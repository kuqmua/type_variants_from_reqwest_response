#[proc_macro_derive(
    TypeVariantsFromReqwestResponse,
    attributes(
        tvtrr_desirable_type,
        tvfrr_100_continue,
        tvfrr_101_switching_protocols,
        tvfrr_102_processing,
        tvfrr_200_ok,
        tvfrr_201_created,
        tvfrr_202_accepted,
        tvfrr_203_non_authoritative_information,
        tvfrr_204_no_content,
        tvfrr_205_reset_content,
        tvfrr_206_partial_content,
        tvfrr_207_multi_status,
        tvfrr_208_already_reported,
        tvfrr_226_im_used,
        tvfrr_300_multiple_choices,
        tvfrr_301_moved_permanently,
        tvfrr_302_found,
        tvfrr_303_see_other,
        tvfrr_304_not_modified,
        tvfrr_305_use_proxy,
        tvfrr_307_temporary_redirect,
        tvfrr_308_permanent_redirect,
        tvfrr_400_bad_request,
        tvfrr_401_unauthorized,
        tvfrr_402_payment_required,
        tvfrr_403_forbidden,
        tvfrr_404_not_found,
        tvfrr_405_method_not_allowed,
        tvfrr_406_not_acceptable,
        tvfrr_407_proxy_authentication_required,
        tvfrr_408_request_timeout,
        tvfrr_409_conflict,
        tvfrr_410_gone,
        tvfrr_411_length_required,
        tvfrr_412_precondition_failed,
        tvfrr_413_payload_too_large,
        tvfrr_414_uri_too_long,
        tvfrr_415_unsupported_media_type,
        tvfrr_416_range_not_satisfiable,
        tvfrr_417_expectation_failed,
        tvfrr_418_im_a_teapot,
        tvfrr_421_misdirected_request,
        tvfrr_422_unprocessable_entity,
        tvfrr_423_locked,
        tvfrr_424_failed_dependency,
        tvfrr_426_upgrade_required,
        tvfrr_428_precondition_required,
        tvfrr_429_too_many_requests,
        tvfrr_431_request_header_fields_too_large,
        tvfrr_451_unavailable_for_legal_reasons,
        tvfrr_500_internal_server_error,
        tvfrr_501_not_implemented,
        tvfrr_502_bad_gateway,
        tvfrr_503_service_unavailable,
        tvfrr_504_gateway_timeout,
        tvfrr_505_http_version_not_supported,
        tvfrr_506_variant_also_negotiates,
        tvfrr_507_insufficient_storage,
        tvfrr_508_loop_detected,
        tvfrr_510_not_extended,
        tvfrr_511_network_authentication_required,
    )
)]
pub fn type_variants_from_reqwest_response(
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    proc_macro_helpers::panic_location::panic_location(); //panic_location function from https://github.com/kuqmua/proc_macro_helpers
    let macro_name = "TypeVariantsFromReqwestResponse";
    let ast: syn::DeriveInput = syn::parse(input).unwrap_or_else(|_| {
        panic!("{macro_name} {}", proc_macro_helpers::global_variables::hardcode::AST_PARSE_FAILED)
    });
    let ident = &ast.ident;
    let data_enum = if let syn::Data::Enum(data_enum) = ast.data {
        data_enum
    } else {
        panic!("{macro_name} {ident} syn::Data is not a syn::Data::Enum");
    };
    let variants_len = data_enum.variants.len();
    let tvtrr_desirable_type_stringified = "tvtrr_desirable_type";
    let mut option_desirable_type  = None;
    data_enum.variants.iter().for_each(|variant|{
        match &variant.fields {
            syn::Fields::Named(_) => {
                variant.attrs.iter().for_each(|attr| {
                    if let true = attr.path.segments[0].ident == tvtrr_desirable_type_stringified {
                        panic!("{macro_name} {ident} attribute {tvtrr_desirable_type_stringified} are not allowed in syn::Fields::Named variants");
                    }
                });
            },
            syn::Fields::Unnamed(fields_unnamed) => {
                variant.attrs.iter().for_each(|attr| {
                    if let true = attr.path.segments[0].ident == tvtrr_desirable_type_stringified {
                        match option_desirable_type {
                            Some(_) => panic!("{macro_name} {ident} only one {tvtrr_desirable_type_stringified} attribute supported"),
                            None => {
                                let unnamed = &fields_unnamed.unnamed;
                                if let true = unnamed.len() == 1  {
                                    option_desirable_type = Some(unnamed[0].ty.clone());
                                }
                                else {
                                    panic!("{macro_name} {ident} variant fields_unnamed.len() != 1");
                                }
                            },
                        }
                    }
                });
            },
            syn::Fields::Unit => panic!("{macro_name} {ident} syn::Data is not a syn::Data::Enum"),
        }
    });
    // println!("{option_desirable_type}");

    
    //
    let (unique_status_codes, variants_with_status_code, variants_from_status_code) =
        data_enum.variants.into_iter().fold(
            (
                Vec::with_capacity(variants_len),
                Vec::with_capacity(variants_len),
                Vec::with_capacity(variants_len),
            ),
            |mut acc, variant| {
                let mut option_attribute = None;
                variant.attrs.iter().for_each(|attr| {
                    if let true = attr.path.segments.len() == 1 {
                        if let Ok(named_attribute) =
                            Attribute::try_from(&attr.path.segments[0].ident)
                        {
                            if let true = option_attribute.is_some() {
                                panic!(
                                    "{macro_name} {ident} duplicated attributes are not supported"
                                );
                            } else {
                                option_attribute = Some(named_attribute);
                            }
                        }
                    }
                });
                let attr = if let Some(attr) = option_attribute {
                    attr
                } else {
                    panic!("{macro_name} {ident} no supported attribute");
                };
                if !acc.0.contains(&attr) {
                    acc.0.push(attr.clone())
                }
                acc.2.push({
                    let variant_ident = &variant.ident;
                    let http_status_code_token_stream = attr.to_http_status_code_quote();
                    match &variant.fields {
                        syn::Fields::Named(fields_named) => {
                            let fields_token_stream = fields_named.named.iter().map(|field| {
                                let field_ident = field.ident.clone().unwrap_or_else(|| {
                                    panic!("{macro_name} {ident} named field ident is None");
                                });
                                quote::quote! { #field_ident: _ }
                            });
                            quote::quote! {
                                #ident::#variant_ident {
                                     #(#fields_token_stream),*
                                } => #http_status_code_token_stream
                            }
                        }
                        syn::Fields::Unnamed(fields_unnamed) => {
                            let fields_token_stream = if let true = fields_unnamed.unnamed.len() == 1 {
                                quote::quote! { _ }
                            }
                            else {
                                panic!("{macro_name} {ident} fields_unnamed.unnamed.len() != 1");                                
                            };
                            quote::quote! {
                                #ident::#variant_ident(#fields_token_stream) => #http_status_code_token_stream
                            }
                        }
                        syn::Fields::Unit => {
                            panic!("{macro_name} {ident} syn::Data is not a syn::Data::Enum")
                        }
                    }
                });
                acc.1.push((attr, variant));
                acc
            },
        );
    let unique_status_codes_len = unique_status_codes.len();
    if let true = unique_status_codes.is_empty() {
        panic!("{macro_name} {ident} true = unique_status_codes.is_empty()");
    }
    let unique_status_codes_len_minus_one = unique_status_codes_len - 1;
    let mut is_last_element_found = false;
    let (status_codes_enums_with_from_impl, status_code_enums_try_from) = unique_status_codes
        .iter()
        .enumerate()
        .fold(
            (
                Vec::with_capacity(unique_status_codes_len),
                Vec::with_capacity(unique_status_codes_len),
            ),
            |mut acc, (index, status_code_attribute)| 
        {
            let status_code_enum_name_stringified = format!("{ident}{status_code_attribute}");
            let status_code_enum_name_token_stream = status_code_enum_name_stringified
            .parse::<proc_macro2::TokenStream>()
            .unwrap_or_else(|_| panic!("{macro_name} {ident} {status_code_enum_name_stringified} {}", proc_macro_helpers::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE));
            let status_code_variants_vec = variants_with_status_code.iter().fold(
                Vec::with_capacity(variants_len),
                |mut acc, (attribute, variant)| {
                    if let true = status_code_attribute == attribute {
                        acc.push(variant);
                    }
                    acc
                },
            );
            let status_code_variants_vec_len = status_code_variants_vec.len();
            let (status_code_variants_vec_token_stream, status_code_variants_vec_from_token_stream)  = status_code_variants_vec
                .iter()
                .fold(
                    (
                        Vec::with_capacity(status_code_variants_vec_len),
                        Vec::with_capacity(status_code_variants_vec_len),
                    ),
                    |mut acc, variant| {
                        let variant_ident = &variant.ident;
                        match &variant.fields {
                            syn::Fields::Named(fields_named) => {
                                let (enum_fields_token_stream, from_enum_fields_token_stream) = fields_named.named.iter().fold(
                                    (
                                        Vec::with_capacity(fields_named.named.len()),
                                        Vec::with_capacity(fields_named.named.len()),
                                    ),
                                    |mut acc, field| {
                                    let field_ident = field.ident.clone().unwrap_or_else(|| {
                                            panic!("{macro_name} {ident} named field ident is None");
                                        });
                                    let field_ty = &field.ty;
                                    acc.0.push(quote::quote! { #field_ident: #field_ty });
                                    acc.1.push(quote::quote! { #field_ident });
                                    acc
                                });
                                acc.0.push(quote::quote! {
                                        #variant_ident {
                                            #(#enum_fields_token_stream),*
                                        }
                                    });
                                acc.1.push(quote::quote! {
                                    #status_code_enum_name_token_stream::#variant_ident {
                                        #(#from_enum_fields_token_stream),*
                                    } => Self::#variant_ident {
                                        #(#from_enum_fields_token_stream),*
                                    }
                                });
                                acc
                            },
                            syn::Fields::Unnamed(fields_unnamed) => {
                                let fields_token_stream = if let true = fields_unnamed.unnamed.len() == 1 {
                                    let field_ty = &fields_unnamed.unnamed[0].ty;
                                    quote::quote! { #field_ty }
                                }
                                else {
                                    panic!("{macro_name} {ident} fields_unnamed.unnamed.len() != 1");                                       
                                };
                                acc.0.push(quote::quote! {
                                        #variant_ident(#fields_token_stream)
                                    });
                                acc.1.push(quote::quote! {
                                    #status_code_enum_name_token_stream::#variant_ident(i) => Self::#variant_ident(i)
                                });
                                acc
                            },
                            syn::Fields::Unit => {
                                panic!("{macro_name} {ident} syn::Data is not a syn::Data::Enum")
                            }
                        }
                    }
                );
            acc.0.push(quote::quote! {
                #[derive(Debug, serde::Serialize, serde::Deserialize)]
                enum #status_code_enum_name_token_stream {
                    #(#status_code_variants_vec_token_stream),*
                }
                impl std::convert::From<#status_code_enum_name_token_stream> for #ident {
                    fn from(value: #status_code_enum_name_token_stream) -> Self {
                        match value {
                            #(#status_code_variants_vec_from_token_stream),*
                        }
                    }
                }
            });
            let http_status_code_token_stream = status_code_attribute.to_http_status_code_quote();
            match index {
                0 => {
                    acc.1.push(quote::quote! {
                        if status_code == #http_status_code_token_stream {
                            match futures::executor::block_on(response.json::<#status_code_enum_name_token_stream>()) {
                                Ok(value) => Ok(#ident::from(value)),
                                Err(e) => Err(
                                    crate::common::api_request_unexpected_error::ApiRequestUnexpectedError::DeserializeBody {
                                        reqwest: e,
                                        status_code,
                                    },
                                ),
                            }
                        }
                    });
                },
                _ => match index == unique_status_codes_len_minus_one{
                    true => {
                        is_last_element_found = true;
                        acc.1.push(quote::quote! {
                            else {
                                Err(
                                    crate::common::api_request_unexpected_error::ApiRequestUnexpectedError::StatusCode {
                                        status_code,
                                    },
                                )
                            }
                        });
                    },
                    false => {
                        acc.1.push(quote::quote! {
                            else if status_code == #http_status_code_token_stream {
                                match futures::executor::block_on(
                                    response.json::<#status_code_enum_name_token_stream>(),
                                ) {
                                    Ok(value) => Ok(#ident::from(value)),
                                    Err(e) => Err(
                                        crate::common::api_request_unexpected_error::ApiRequestUnexpectedError::DeserializeBody{
                                            reqwest: e,
                                            status_code,
                                        },
                                    ),
                                }
                            }
                        });
                    },
                }
            }
            acc
        });
    if let false = is_last_element_found {
        panic!("{macro_name} {ident} false = is_last_element_found");
    }
    let gen = quote::quote! {
        impl std::convert::From<&#ident> for http::StatusCode {
            fn from(value: &#ident) -> Self {
                match value {
                    #(#variants_from_status_code),*
                }
            }
        }
        #(#status_codes_enums_with_from_impl)*
        impl std::convert::TryFrom<reqwest::Response> for #ident {
            type Error = crate::common::api_request_unexpected_error::ApiRequestUnexpectedError;
            fn try_from(response: reqwest::Response) -> Result<Self, Self::Error> {
                let status_code = response.status();
                #(#status_code_enums_try_from)*
            }
        }
    };
    // if ident == "" {
    //     println!("{gen}");
    // }
    gen.into()
}

#[derive(
    Debug,
    strum_macros::EnumIter,
    strum_macros::Display,
    enum_extension::EnumExtension,
    PartialEq,
    Eq,
    Clone,
)]
enum Attribute {
    Tvfrr100Continue,
    Tvfrr101SwitchingProtocols,
    Tvfrr102Processing,
    Tvfrr200Ok,
    Tvfrr201Created,
    Tvfrr202Accepted,
    Tvfrr203NonAuthoritativeInformation,
    Tvfrr204NoContent,
    Tvfrr205ResetContent,
    Tvfrr206PartialContent,
    Tvfrr207MultiStatus,
    Tvfrr208AlreadyReported,
    Tvfrr226ImUsed,
    Tvfrr300MultipleChoices,
    Tvfrr301MovedPermanently,
    Tvfrr302Found,
    Tvfrr303SeeOther,
    Tvfrr304NotModified,
    Tvfrr305UseProxy,
    Tvfrr307TemporaryRedirect,
    Tvfrr308PermanentRedirect,
    Tvfrr400BadRequest,
    Tvfrr401Unauthorized,
    Tvfrr402PaymentRequired,
    Tvfrr403Forbidden,
    Tvfrr404NotFound,
    Tvfrr405MethodNotAllowed,
    Tvfrr406NotAcceptable,
    Tvfrr407ProxyAuthenticationRequired,
    Tvfrr408RequestTimeout,
    Tvfrr409Conflict,
    Tvfrr410Gone,
    Tvfrr411LengthRequired,
    Tvfrr412PreconditionFailed,
    Tvfrr413PayloadTooLarge,
    Tvfrr414UriTooLong,
    Tvfrr415UnsupportedMediaType,
    Tvfrr416RangeNotSatisfiable,
    Tvfrr417ExpectationFailed,
    Tvfrr418ImATeapot,
    Tvfrr421MisdirectedRequest,
    Tvfrr422UnprocessableEntity,
    Tvfrr423Locked,
    Tvfrr424FailedDependency,
    Tvfrr426UpgradeRequired,
    Tvfrr428PreconditionRequired,
    Tvfrr429TooManyRequests,
    Tvfrr431RequestHeaderFieldsTooLarge,
    Tvfrr451UnavailableForLegalReasons,
    Tvfrr500InternalServerError,
    Tvfrr501NotImplemented,
    Tvfrr502BadGateway,
    Tvfrr503ServiceUnavailable,
    Tvfrr504GatewayTimeout,
    Tvfrr505HttpVersionNotSupported,
    Tvfrr506VariantAlsoNegotiates,
    Tvfrr507InsufficientStorage,
    Tvfrr508LoopDetected,
    Tvfrr510NotExtended,
    Tvfrr511NetworkAuthenticationRequired,
}

impl Attribute {
    fn to_http_status_code_quote(&self) -> proc_macro2::TokenStream {
        match self {
            Attribute::Tvfrr100Continue => quote::quote! {http::StatusCode::CONTINUE},
            Attribute::Tvfrr101SwitchingProtocols => {
                quote::quote! {http::StatusCode::SWITCHING_PROTOCOLS}
            }
            Attribute::Tvfrr102Processing => quote::quote! {http::StatusCode::PROCESSING},
            Attribute::Tvfrr200Ok => quote::quote! {http::StatusCode::OK},
            Attribute::Tvfrr201Created => quote::quote! {http::StatusCode::CREATED},
            Attribute::Tvfrr202Accepted => quote::quote! {http::StatusCode::ACCEPTED},
            Attribute::Tvfrr203NonAuthoritativeInformation => {
                quote::quote! {http::StatusCode::NON_AUTHORITATIVE_INFORMATION}
            }
            Attribute::Tvfrr204NoContent => quote::quote! {http::StatusCode::NO_CONTENT},
            Attribute::Tvfrr205ResetContent => quote::quote! {http::StatusCode::RESET_CONTENT},
            Attribute::Tvfrr206PartialContent => {
                quote::quote! {http::StatusCode::PARTIAL_CONTENT}
            }
            Attribute::Tvfrr207MultiStatus => quote::quote! {http::StatusCode::MULTI_STATUS},
            Attribute::Tvfrr208AlreadyReported => {
                quote::quote! {http::StatusCode::ALREADY_REPORTED}
            }
            Attribute::Tvfrr226ImUsed => quote::quote! {http::StatusCode::IM_USED},
            Attribute::Tvfrr300MultipleChoices => {
                quote::quote! {http::StatusCode::MULTIPLE_CHOICES}
            }
            Attribute::Tvfrr301MovedPermanently => {
                quote::quote! {http::StatusCode::MOVED_PERMANENTLY}
            }
            Attribute::Tvfrr302Found => quote::quote! {http::StatusCode::FOUND},
            Attribute::Tvfrr303SeeOther => quote::quote! {http::StatusCode::SEE_OTHER},
            Attribute::Tvfrr304NotModified => quote::quote! {http::StatusCode::NOT_MODIFIED},
            Attribute::Tvfrr305UseProxy => quote::quote! {http::StatusCode::USE_PROXY},
            Attribute::Tvfrr307TemporaryRedirect => {
                quote::quote! {http::StatusCode::TEMPORARY_REDIRECT}
            }
            Attribute::Tvfrr308PermanentRedirect => {
                quote::quote! {http::StatusCode::PERMANENT_REDIRECT}
            }
            Attribute::Tvfrr400BadRequest => quote::quote! {http::StatusCode::BAD_REQUEST},
            Attribute::Tvfrr401Unauthorized => quote::quote! {http::StatusCode::UNAUTHORIZED},
            Attribute::Tvfrr402PaymentRequired => {
                quote::quote! {http::StatusCode::PAYMENT_REQUIRED}
            }
            Attribute::Tvfrr403Forbidden => quote::quote! {http::StatusCode::FORBIDDEN},
            Attribute::Tvfrr404NotFound => quote::quote! {http::StatusCode::NOT_FOUND},
            Attribute::Tvfrr405MethodNotAllowed => {
                quote::quote! {http::StatusCode::METHOD_NOT_ALLOWED}
            }
            Attribute::Tvfrr406NotAcceptable => quote::quote! {http::StatusCode::NOT_ACCEPTABLE},
            Attribute::Tvfrr407ProxyAuthenticationRequired => {
                quote::quote! {http::StatusCode::PROXY_AUTHENTICATION_REQUIRED}
            }
            Attribute::Tvfrr408RequestTimeout => {
                quote::quote! {http::StatusCode::REQUEST_TIMEOUT}
            }
            Attribute::Tvfrr409Conflict => quote::quote! {http::StatusCode::CONFLICT},
            Attribute::Tvfrr410Gone => quote::quote! {http::StatusCode::GONE},
            Attribute::Tvfrr411LengthRequired => {
                quote::quote! {http::StatusCode::LENGTH_REQUIRED}
            }
            Attribute::Tvfrr412PreconditionFailed => {
                quote::quote! {http::StatusCode::PRECONDITION_FAILED}
            }
            Attribute::Tvfrr413PayloadTooLarge => {
                quote::quote! {http::StatusCode::PAYLOAD_TOO_LARGE}
            }
            Attribute::Tvfrr414UriTooLong => quote::quote! {http::StatusCode::URI_TOO_LONG},
            Attribute::Tvfrr415UnsupportedMediaType => {
                quote::quote! {http::StatusCode::UNSUPPORTED_MEDIA_TYPE}
            }
            Attribute::Tvfrr416RangeNotSatisfiable => {
                quote::quote! {http::StatusCode::RANGE_NOT_SATISFIABLE}
            }
            Attribute::Tvfrr417ExpectationFailed => {
                quote::quote! {http::StatusCode::EXPECTATION_FAILED}
            }
            Attribute::Tvfrr418ImATeapot => quote::quote! {http::StatusCode::IM_A_TEAPOT},
            Attribute::Tvfrr421MisdirectedRequest => {
                quote::quote! {http::StatusCode::MISDIRECTED_REQUEST}
            }
            Attribute::Tvfrr422UnprocessableEntity => {
                quote::quote! {http::StatusCode::UNPROCESSABLE_ENTITY}
            }
            Attribute::Tvfrr423Locked => quote::quote! {http::StatusCode::LOCKED},
            Attribute::Tvfrr424FailedDependency => {
                quote::quote! {http::StatusCode::FAILED_DEPENDENCY}
            }
            Attribute::Tvfrr426UpgradeRequired => {
                quote::quote! {http::StatusCode::UPGRADE_REQUIRED}
            }
            Attribute::Tvfrr428PreconditionRequired => {
                quote::quote! {http::StatusCode::PRECONDITION_REQUIRED}
            }
            Attribute::Tvfrr429TooManyRequests => {
                quote::quote! {http::StatusCode::TOO_MANY_REQUESTS}
            }
            Attribute::Tvfrr431RequestHeaderFieldsTooLarge => {
                quote::quote! {http::StatusCode::REQUEST_HEADER_FIELDS_TOO_LARGE}
            }
            Attribute::Tvfrr451UnavailableForLegalReasons => {
                quote::quote! {http::StatusCode::UNAVAILABLE_FOR_LEGAL_REASONS}
            }
            Attribute::Tvfrr500InternalServerError => {
                quote::quote! {http::StatusCode::INTERNAL_SERVER_ERROR}
            }
            Attribute::Tvfrr501NotImplemented => {
                quote::quote! {http::StatusCode::NOT_IMPLEMENTED}
            }
            Attribute::Tvfrr502BadGateway => quote::quote! {http::StatusCode::BAD_GATEWAY},
            Attribute::Tvfrr503ServiceUnavailable => {
                quote::quote! {http::StatusCode::SERVICE_UNAVAILABLE}
            }
            Attribute::Tvfrr504GatewayTimeout => {
                quote::quote! {http::StatusCode::GATEWAY_TIMEOUT}
            }
            Attribute::Tvfrr505HttpVersionNotSupported => {
                quote::quote! {http::StatusCode::HTTP_VERSION_NOT_SUPPORTED}
            }
            Attribute::Tvfrr506VariantAlsoNegotiates => {
                quote::quote! {http::StatusCode::VARIANT_ALSO_NEGOTIATES}
            }
            Attribute::Tvfrr507InsufficientStorage => {
                quote::quote! {http::StatusCode::INSUFFICIENT_STORAGE}
            }
            Attribute::Tvfrr508LoopDetected => quote::quote! {http::StatusCode::LOOP_DETECTED},
            Attribute::Tvfrr510NotExtended => quote::quote! {http::StatusCode::NOT_EXTENDED},
            Attribute::Tvfrr511NetworkAuthenticationRequired => {
                quote::quote! {http::StatusCode::NETWORK_AUTHENTICATION_REQUIRED}
            }
        }
    }
}

impl TryFrom<&syn::Ident> for Attribute {
    type Error = ();
    fn try_from(value: &syn::Ident) -> Result<Self, Self::Error> {
        if value == "tvfrr_100_continue" {
            Ok(Attribute::Tvfrr100Continue)
        } else if value == "tvfrr_101_switching_protocols" {
            Ok(Attribute::Tvfrr101SwitchingProtocols)
        } else if value == "tvfrr_102_processing" {
            Ok(Attribute::Tvfrr102Processing)
        } else if value == "tvfrr_200_ok" {
            Ok(Attribute::Tvfrr200Ok)
        } else if value == "tvfrr_201_created" {
            Ok(Attribute::Tvfrr201Created)
        } else if value == "tvfrr_202_accepted" {
            Ok(Attribute::Tvfrr202Accepted)
        } else if value == "tvfrr_203_non_authoritative_information" {
            Ok(Attribute::Tvfrr203NonAuthoritativeInformation)
        } else if value == "tvfrr_204_no_content" {
            Ok(Attribute::Tvfrr204NoContent)
        } else if value == "tvfrr_205_reset_content" {
            Ok(Attribute::Tvfrr205ResetContent)
        } else if value == "tvfrr_206_partial_content" {
            Ok(Attribute::Tvfrr206PartialContent)
        } else if value == "tvfrr_207_multi_status" {
            Ok(Attribute::Tvfrr207MultiStatus)
        } else if value == "tvfrr_208_already_reported" {
            Ok(Attribute::Tvfrr208AlreadyReported)
        } else if value == "tvfrr_226_im_used" {
            Ok(Attribute::Tvfrr226ImUsed)
        } else if value == "tvfrr_300_multiple_choices" {
            Ok(Attribute::Tvfrr300MultipleChoices)
        } else if value == "tvfrr_301_moved_permanently" {
            Ok(Attribute::Tvfrr301MovedPermanently)
        } else if value == "tvfrr_302_found" {
            Ok(Attribute::Tvfrr302Found)
        } else if value == "tvfrr_303_see_other" {
            Ok(Attribute::Tvfrr303SeeOther)
        } else if value == "tvfrr_304_not_modified" {
            Ok(Attribute::Tvfrr304NotModified)
        } else if value == "tvfrr_305_use_proxy" {
            Ok(Attribute::Tvfrr305UseProxy)
        } else if value == "tvfrr_307_temporary_redirect" {
            Ok(Attribute::Tvfrr307TemporaryRedirect)
        } else if value == "tvfrr_308_permanent_redirect" {
            Ok(Attribute::Tvfrr308PermanentRedirect)
        } else if value == "tvfrr_400_bad_request" {
            Ok(Attribute::Tvfrr400BadRequest)
        } else if value == "tvfrr_401_unauthorized" {
            Ok(Attribute::Tvfrr401Unauthorized)
        } else if value == "tvfrr_402_payment_required" {
            Ok(Attribute::Tvfrr402PaymentRequired)
        } else if value == "tvfrr_403_forbidden" {
            Ok(Attribute::Tvfrr403Forbidden)
        } else if value == "tvfrr_404_not_found" {
            Ok(Attribute::Tvfrr404NotFound)
        } else if value == "tvfrr_405_method_not_allowed" {
            Ok(Attribute::Tvfrr405MethodNotAllowed)
        } else if value == "tvfrr_406_not_acceptable" {
            Ok(Attribute::Tvfrr406NotAcceptable)
        } else if value == "tvfrr_407_proxy_authentication_required" {
            Ok(Attribute::Tvfrr407ProxyAuthenticationRequired)
        } else if value == "tvfrr_408_request_timeout" {
            Ok(Attribute::Tvfrr408RequestTimeout)
        } else if value == "tvfrr_409_conflict" {
            Ok(Attribute::Tvfrr409Conflict)
        } else if value == "tvfrr_410_gone" {
            Ok(Attribute::Tvfrr410Gone)
        } else if value == "tvfrr_411_length_required" {
            Ok(Attribute::Tvfrr411LengthRequired)
        } else if value == "tvfrr_412_precondition_failed" {
            Ok(Attribute::Tvfrr412PreconditionFailed)
        } else if value == "tvfrr_413_payload_too_large" {
            Ok(Attribute::Tvfrr413PayloadTooLarge)
        } else if value == "tvfrr_414_uri_too_long" {
            Ok(Attribute::Tvfrr414UriTooLong)
        } else if value == "tvfrr_415_unsupported_media_type" {
            Ok(Attribute::Tvfrr415UnsupportedMediaType)
        } else if value == "tvfrr_416_range_not_satisfiable" {
            Ok(Attribute::Tvfrr416RangeNotSatisfiable)
        } else if value == "tvfrr_417_expectation_failed" {
            Ok(Attribute::Tvfrr417ExpectationFailed)
        } else if value == "tvfrr_418_im_a_teapot" {
            Ok(Attribute::Tvfrr418ImATeapot)
        } else if value == "tvfrr_421_misdirected_request" {
            Ok(Attribute::Tvfrr421MisdirectedRequest)
        } else if value == "tvfrr_422_unprocessable_entity" {
            Ok(Attribute::Tvfrr422UnprocessableEntity)
        } else if value == "tvfrr_423_locked" {
            Ok(Attribute::Tvfrr423Locked)
        } else if value == "tvfrr_424_failed_dependency" {
            Ok(Attribute::Tvfrr424FailedDependency)
        } else if value == "tvfrr_426_upgrade_required" {
            Ok(Attribute::Tvfrr426UpgradeRequired)
        } else if value == "tvfrr_428_precondition_required" {
            Ok(Attribute::Tvfrr428PreconditionRequired)
        } else if value == "tvfrr_429_too_many_requests" {
            Ok(Attribute::Tvfrr429TooManyRequests)
        } else if value == "tvfrr_431_request_header_fields_too_large" {
            Ok(Attribute::Tvfrr431RequestHeaderFieldsTooLarge)
        } else if value == "tvfrr_451_unavailable_for_legal_reasons" {
            Ok(Attribute::Tvfrr451UnavailableForLegalReasons)
        } else if value == "tvfrr_500_internal_server_error" {
            Ok(Attribute::Tvfrr500InternalServerError)
        } else if value == "tvfrr_501_not_implemented" {
            Ok(Attribute::Tvfrr501NotImplemented)
        } else if value == "tvfrr_502_bad_gateway" {
            Ok(Attribute::Tvfrr502BadGateway)
        } else if value == "tvfrr_503_service_unavailable" {
            Ok(Attribute::Tvfrr503ServiceUnavailable)
        } else if value == "tvfrr_504_gateway_timeout" {
            Ok(Attribute::Tvfrr504GatewayTimeout)
        } else if value == "tvfrr_505_http_version_not_supported" {
            Ok(Attribute::Tvfrr505HttpVersionNotSupported)
        } else if value == "tvfrr_506_variant_also_negotiates" {
            Ok(Attribute::Tvfrr506VariantAlsoNegotiates)
        } else if value == "tvfrr_507_insufficient_storage" {
            Ok(Attribute::Tvfrr507InsufficientStorage)
        } else if value == "tvfrr_508_loop_detected" {
            Ok(Attribute::Tvfrr508LoopDetected)
        } else if value == "tvfrr_510_not_extended" {
            Ok(Attribute::Tvfrr510NotExtended)
        } else if value == "tvfrr_511_network_authentication_required" {
            Ok(Attribute::Tvfrr511NetworkAuthenticationRequired)
        } else {
            Err(())
        }
    }
}
