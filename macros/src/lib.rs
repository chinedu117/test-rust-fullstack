use proc_macro::{TokenStream};
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput};

#[proc_macro_derive(GenerateCrudRoutes)]
pub fn create_functions(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = input.ident;

    let output = quote! {
        impl #struct_name {
            fn default_err(err: DbErr) -> HttpResponse {
                ApiError { kind: ApiErrorType::BadClientData, msg: err.to_string() }.error_response()
            }

            async fn list(data: web::Data<AppState>) -> HttpResponse {
                match Entity.select().all(&data.db).await {
                    Ok(list) => { HttpResponse::Ok().json(list) }
                    Err(err) => Self::default_err(err)
                }
            }

            async fn get(path: web::Path<(i32,)>, data: web::Data<AppState>) -> HttpResponse {
                let id: i32 = path.into_inner().0;
                match  Entity::find_by_id(id).one(&data.db).await {
                    Ok(record) => {
                        match record {
                            None => ApiError { kind: ApiErrorType::NotFound, msg: "".to_string() }.error_response(),
                            Some(r) => HttpResponse::Ok().json(r)
                        }
                    }
                    Err(err) => Self::default_err(err)
                }
            }

            async fn update(path: web::Path<(i32,)>, record: web::Json<ModelWithoutId>, data: web::Data<AppState>) -> HttpResponse {
                let id: i32 = path.into_inner().0;
                match Entity::find_by_id(id).one(&data.db).await {
                    Ok(u) => {
                        match u {
                            None => ApiError { kind: ApiErrorType::NotFound, msg: "".to_string() }.error_response(),
                            Some(_) => {
                                let mut updated_record = record.into_inner().clone().into_active_model();
                                updated_record.set(Column::Id, Value::Int(Some(id)));
                                match updated_record.save(&data.db).await {
                                    Ok(_) => HttpResponse::Ok().status(StatusCode::ACCEPTED).json(""),
                                    Err(err) => Self::default_err(err)
                                }
                            }
                        }

                    },
                    Err(err) => {
                        return Self::default_err(err);
                    }
                }
            }

            async fn delete(path: web::Path<(i32,)>, data: web::Data<AppState>) -> HttpResponse {
                let id: i32 = path.into_inner().0;
                match Entity::find_by_id(id).one(&data.db).await {
                    Ok(r) => {
                        match r {
                            None => Self::default_err(DbErr::Custom(format!("Cound not find id: {}", id).to_string())),
                            Some(v) => {
                                let m: ActiveModel  = v.into();
                                match m.delete(&data.db).await {
                                    Ok(_) => HttpResponse::Ok().status(StatusCode::ACCEPTED).body(""),
                                    Err(err) => Self::default_err(err)
                                }
                            }
                        }
                    }
                    Err(err) => Self::default_err(err)
                }
            }

            async fn create(record: web::Json<ModelWithoutId>, data: web::Data<AppState>) -> HttpResponse {
                let new_record = record.into_inner().into_active_model();
                match new_record.save(&data.db).await {
                    Ok(r) => {
                        match r.get_primary_key_value() {
                            None =>  HttpResponse::Ok().json(""),
                            Some(id) => {
                                match id {
                                    ValueTuple::One(i) =>  HttpResponse::Ok().json(i.to_string()),
                                    _ => HttpResponse::Ok().json("")
                                }
                            }
                        }
                    },
                    Err(err) => Self::default_err(err)
                }
            }
        }
    };
    TokenStream::from(output)
}

#[proc_macro_derive(GenerateModels)]
pub fn create_models(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let fields = match input.data {        
        Data::Struct(data_struct) => data_struct.fields,
        _ => panic!("This macro can only be used on structs!"),
    };

    let mut field_code = Vec::new();
    fields.iter().for_each(|field| {
        let field_name = field.ident.as_ref().unwrap();
        let field_type = &field.ty;        
        field_code.push(quote! {
            pub #field_name : #field_type,
        });
    });

    

    let mut field_code_without_id = Vec::new();
     fields.iter().for_each(|field| {
        let field_name = field.ident.as_ref().unwrap();
        let field_type = &field.ty;        
        if field_name.to_string() != "id" {
            field_code_without_id.push(quote! {
                pub #field_name : #field_type,
            });
        }            
    });

    

    let struct_name = input.ident;

    let modified_name_ui = syn::Ident::new(
        &format!("{}{}", struct_name, "Ui"),
        struct_name.span()
    );

    let lower_case_name = syn::Ident::new(
        &struct_name.to_string().to_lowercase(),
        struct_name.span()
    );

    let table_name = format!("{}", lower_case_name);
    
    let expanded = quote! {
        #[derive(PartialEq, Deserialize, Serialize, Default, Properties, Clone, Reflect)]
        pub struct #modified_name_ui {
            #(#field_code)*
        }
            
        #[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
        #[sea_orm(table_name = #table_name)]
        pub struct Model {
            #[sea_orm(primary_key)]
            #(#field_code)*
        }
        #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, DeriveIntoActiveModel)]
        pub struct ModelWithoutId {
            #(#field_code_without_id)*
        }
                

    };
    TokenStream::from(expanded)
}

