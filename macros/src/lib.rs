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

#[proc_macro_derive(CrudRoutes)]
pub fn create_routes(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = input.ident;

    let output = quote! {
        use axum::extract::{Path, State};
        use axum::http::StatusCode;
        use axum_macros::*;
        use axum::response::{IntoResponse};
        use sea_orm::{ActiveModelTrait, DbErr, EntityOrSelect, EntityTrait, IntoActiveModel, Value};
        use sea_orm::sea_query::ValueTuple;
        use crate::services::error_handler::{ApiError, ApiErrorType};
        impl #struct_name {
            async fn list(State(state): State<AppState>) -> impl IntoResponse {
                let list = Entity.select().all(&state.db).await;
                match list {
                    Ok(list) => {
                        Json(list).into_response()
                    }
                    Err(err) => {
                        let err = ApiError {
                            kind: ApiErrorType::InternalError,
                            msg: err.to_string()
                        };
                        err.into_response()
                    }
                }
            }

            async fn get(State(state): State<AppState>, Path(id): Path<i32>) -> impl IntoResponse {
                match  Entity::find_by_id(id).one(&state.db).await {
                    Ok(record) => {
                        match record {
                            None => {
                                let error = ApiError { kind: ApiErrorType::NotFound, msg: "".to_string() };
                                error.into_response()
                            },
                            Some(r) => Json(r).into_response()
                        }
                    }
                    Err(err) => {
                        ApiError { kind: ApiErrorType::InternalError, msg: err.to_string() }.into_response()
                    }
                }
            }

            async fn create(State(state): State<AppState>, Json(record): Json<ModelWithoutId>) -> impl IntoResponse {
                let new_record = record.into_active_model();
                match new_record.save(&state.db).await {
                    Ok(r) => {
                        match r.get_primary_key_value() {
                            None =>  Json("").into_response(),
                            Some(id) => {
                                match id {
                                    ValueTuple::One(i) =>  Json(i.to_string()).into_response(),
                                    _ => Json("").into_response()
                                }
                            }
                        }
                    },
                    Err(err) => {
                        ApiError { kind: ApiErrorType::InternalError, msg: err.to_string() }.into_response()
                    }
                }
            }

            
            async fn update(State(state): State<AppState>, Path(id): Path<i32>, Json(record): Json<ModelWithoutId>) -> impl IntoResponse {
                match Entity::find_by_id(id).one(&state.db).await {
                    Ok(u) => {
                        match u {
                            None => ApiError { kind: ApiErrorType::NotFound, msg: "".to_string() }.into_response(),
                            Some(_) => {
                                let mut updated_record = record.into_active_model();
                                updated_record.set(Column::Id, Value::Int(Some(id)));
                                match updated_record.save(&state.db).await {
                                    
                                    Ok(_) => StatusCode::ACCEPTED.into_response(),
                                    
                                    Err(err) => ApiError { 
                                        kind: ApiErrorType::InternalError, 
                                        msg: err.to_string() 
                                    }.into_response()
                                }
                            }
                        }

                    },
                    Err(err) => {
                        ApiError { kind: ApiErrorType::InternalError, msg: err.to_string() }.into_response()
                    }
                }
            }

            async fn delete(State(state): State<AppState>, Path(id): Path<i32>) -> impl IntoResponse {
                match Entity::find_by_id(id).one(&state.db).await {
                    Ok(r) => {
                        match r {
                            None => {
                                let error = ApiError { kind: ApiErrorType::NotFound, msg: "".to_string() };
                                error.into_response()
                            },
                            Some(v) => {
                                let m: ActiveModel  = v.into();
                                match m.delete(&state.db).await {
                                    Ok(_) => StatusCode::ACCEPTED.into_response(),
                                    Err(err) => {
                                        ApiError { kind: ApiErrorType::InternalError, msg: err.to_string() }.into_response()
                                    }
                                }
                            }
                        }
                    }
                    Err(err) => {
                        ApiError { kind: ApiErrorType::InternalError, msg: err.to_string() }.into_response()
                    }
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
        #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, DeriveIntoActiveModel, Default)]
        pub struct ModelWithoutId {
            #(#field_code_without_id)*
        }
                

    };
    TokenStream::from(expanded)
}

