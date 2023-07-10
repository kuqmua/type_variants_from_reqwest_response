#[derive(
    Debug,
    strum_macros::Display,
    PartialEq,
    Eq,
    Clone,
    Hash
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

impl TryFrom<&std::string::String> for Attribute {
    type Error = ();
    fn try_from(value: &std::string::String) -> Result<Self, Self::Error> {
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

#[proc_macro_attribute]
pub fn type_variants_from_reqwest_response_attribute(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    item
}

#[proc_macro_derive(
    TypeVariantsFromReqwestResponse,
    attributes(
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
    let proc_macro_name_ident_stringified = format!("{macro_name} {ident}");
    let ident_response_variants_stringified = format!("{ident}ResponseVariants");
    let ident_response_variants_token_stream = ident_response_variants_stringified
    .parse::<proc_macro2::TokenStream>()
    .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {ident_response_variants_stringified} {}", proc_macro_helpers::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE));
    let attribute_path = "type_variants_from_reqwest_response::type_variants_from_reqwest_response_attribute";
    let option_attribute = ast.attrs.into_iter().find(|attr| {
        attribute_path == {
            let mut stringified_path = quote::ToTokens::to_token_stream(&attr.path).to_string();
            stringified_path.retain(|c| !c.is_whitespace());
            stringified_path
        }
    });
    let (
        desirable_type_token_stream,
        desirable_type_status_code_token_stream,
        desirable_type_enum_name,
    ) = if let Some(attribute) = option_attribute {
        let mut stringified_tokens =
            quote::ToTokens::to_token_stream(&attribute.tokens).to_string();
        stringified_tokens.retain(|c| !c.is_whitespace());
        match stringified_tokens.len() > 3 {
            true => {
                let mut chars = stringified_tokens.chars();
                match (chars.next(), chars.last()) {
                        (None, None) => panic!("FromEnum {ident} no first and last token attribute"),
                        (None, Some(_)) => panic!("FromEnum {ident} no first token attribute"),
                        (Some(_), None) => panic!("FromEnum {ident} no last token attribute"),
                        (Some(first), Some(last)) => match (first == '(', last == ')') {
                            (true, true) => {
                                match stringified_tokens.get(1..(stringified_tokens.len()-1)) {
                                    Some(inner_tokens_str) => {
                                        let vec_attr_params = inner_tokens_str.split(',').map(|str|{str.to_string()}).collect::<Vec<std::string::String>>();
                                        if let false = vec_attr_params.len() == 2 {
                                            panic!("{proc_macro_name_ident_stringified} vec_attr_params.len() != 2");
                                        }
                                        match (
                                            vec_attr_params.get(0),
                                            vec_attr_params.get(1)
                                        ) {
                                            (None, None) => panic!("{proc_macro_name_ident_stringified} failed to get vec_attr_params.get(0) or vec_attr_params.get(1)"),
                                            (None, Some(_)) => panic!("{proc_macro_name_ident_stringified} failed to get vec_attr_params.get(0)"),
                                            (Some(_), None) => panic!("{proc_macro_name_ident_stringified} failed to get vec_attr_params.get(1)"),
                                            (Some(first_param), Some(second_param)) => {
                                                let attribute = Attribute::try_from(second_param).unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} second_param failed to Attribute::try_from"));
                                                (
                                                    first_param
                                                    .parse::<proc_macro2::TokenStream>()
                                                    .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {first_param} {}", proc_macro_helpers::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE)),
                                                    attribute.to_http_status_code_quote(),
                                                    {
                                                        let status_code_enum_name_stingified = format!("{ident_response_variants_token_stream}{attribute}");
                                                        status_code_enum_name_stingified
                                                        .parse::<proc_macro2::TokenStream>()
                                                        .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {status_code_enum_name_stingified} {}", proc_macro_helpers::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                                                    }
                                                )
                                            },
                                        }
                                    },
                                    None => panic!("FromEnum {ident} cannot get inner_token"),
                                }
                            },
                            (true, false) => panic!("FromEnum {ident} last token attribute is not )"),
                            (false, true) => panic!("FromEnum {ident} first token attribute is not ("),
                            (false, false) => panic!("FromEnum {ident} first token attribute is not ( and last token attribute is not )"),
                        },
                    }
            }
            false => panic!("FromEnum {ident} {stringified_tokens}.len() > 3 == false"),
        }
    } else {
        panic!("{ident} FromEnum has no {attribute_path}");
    };
    let data_enum = if let syn::Data::Enum(data_enum) = ast.data {
        data_enum
    } else {
        panic!("{proc_macro_name_ident_stringified} syn::Data is not a syn::Data::Enum");
    };
    let unnamed_camel_case = proc_macro_helpers::error_occurence::hardcode::unnamed_camel_case();
    let supported_enum_variant = proc_macro_helpers::error_occurence::supported_enum_variant::create_supported_enum_variant(
        &data_enum,
        &proc_macro_name_ident_stringified,
        &unnamed_camel_case,
    );
    let with_serialize_deserialize_camel_case = proc_macro_helpers::error_occurence::hardcode::with_serialize_deserialize_camel_case();
    let with_serialize_deserialize_lower_case = proc_macro_helpers::error_occurence::hardcode::with_serialize_deserialize_lower_case();
    let error_occurence_lower_case = proc_macro_helpers::error_occurence::hardcode::error_occurence_lower_case();
    let vec_lower_case = proc_macro_helpers::error_occurence::hardcode::vec_lower_case(); 
    let hashmap_lower_case = proc_macro_helpers::error_occurence::hardcode::hashmap_lower_case();
    let key_lower_case = proc_macro_helpers::error_occurence::hardcode::key_lower_case();
    let value_lower_case = proc_macro_helpers::error_occurence::hardcode::value_lower_case();
    let syn_type_path_stringified = proc_macro_helpers::error_occurence::hardcode::syn_type_path_stringified();
    let supports_only_supported_container_stringified = proc_macro_helpers::error_occurence::hardcode::supports_only_supported_container_stringified();
    let generics_len = ast.generics.params.len();
    let enum_with_serialize_deserialize_logic = proc_macro_helpers::error_occurence::generate_with_serialize_deserialize_version::generate_with_serialize_deserialize_version(
        &supported_enum_variant,
        data_enum.variants.iter().map(|variant|{variant.clone()}).collect(),
        &with_serialize_deserialize_lower_case,
        &error_occurence_lower_case,
        &vec_lower_case,
        &hashmap_lower_case,
        &key_lower_case,
        &value_lower_case,
        &proc_macro_name_ident_stringified,
        &syn_type_path_stringified,
        generics_len,
        &supports_only_supported_container_stringified,
        &with_serialize_deserialize_camel_case,
        &unnamed_camel_case,
        &ident_response_variants_token_stream,
        Some(quote::quote!{
            DesirableType(#desirable_type_token_stream)
        }),
        false,
    );
    let variants_len = data_enum.variants.len();
    let try_error_ident_stringified = format!("{ident}WithSerializeDeserialize");
    let try_error_ident_token_stream = try_error_ident_stringified
    .parse::<proc_macro2::TokenStream>()
    .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {try_error_ident_stringified} {}", proc_macro_helpers::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE));
    let (
        unique_status_codes, 
        variants_from_status_code,
        desirable_type_try_from_ident,
    ) = data_enum.variants.clone().into_iter().fold(
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
                        Attribute::try_from(&attr.path.segments[0].ident.to_string())
                    {
                        if let true = option_attribute.is_some() {
                            panic!(
                                "{proc_macro_name_ident_stringified} duplicated attributes are not supported"
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
                panic!("{proc_macro_name_ident_stringified} no supported attribute");
            };
            if !acc.0.contains(&attr) {
                acc.0.push(attr.clone())
            }
            let (
                variants_from_status_code,
                desirable_type_try_from_ident,
            ) = {
                let variant_ident = &variant.ident;
                let http_status_code_token_stream = attr.to_http_status_code_quote();
                match &variant.fields {
                    syn::Fields::Named(fields_named) => {
                        let fields_named_named_len = fields_named.named.len();
                        let (
                            fields_token_stream_variants_from_status_code,
                            fields_token_stream_desirable_type_try_from_ident,
                        ) = fields_named.named.iter().fold(
                            (
                                Vec::with_capacity(fields_named_named_len),
                                Vec::with_capacity(fields_named_named_len),
                            ),
                            |mut acc, field| {
                                let field_ident = field.ident.clone().unwrap_or_else(|| {
                                    panic!("{proc_macro_name_ident_stringified} named field ident is None");
                                });
                                acc.0.push(quote::quote! { #field_ident: _ });
                                acc.1.push(field_ident);
                                acc
                        });
                        (
                            quote::quote! {
                                #ident_response_variants_token_stream::#variant_ident {
                                     #(#fields_token_stream_variants_from_status_code),*
                                } => #http_status_code_token_stream
                            },
                            quote::quote! {
                                #ident_response_variants_token_stream::#variant_ident {
                                     #(#fields_token_stream_desirable_type_try_from_ident),*
                                } => Err(#try_error_ident_token_stream::#variant_ident { #(#fields_token_stream_desirable_type_try_from_ident),* })
                            }
                        )
                    }
                    syn::Fields::Unnamed(fields_unnamed) => {
                        let fields_token_stream = if let true = fields_unnamed.unnamed.len() == 1 {
                            quote::quote! { _ }
                        }
                        else {
                            panic!("{proc_macro_name_ident_stringified} fields_unnamed.unnamed.len() != 1");                           
                        };
                        (
                            quote::quote! {
                                #ident::#variant_ident(#fields_token_stream) => #http_status_code_token_stream
                            },
                            quote::quote! {
                                #ident_response_variants_token_stream::#variant_ident(i) => Ok(i)
                            }
                        )

                    }
                    syn::Fields::Unit => {
                        panic!("{proc_macro_name_ident_stringified} syn::Data is not a syn::Data::Enum")
                    }
                }
            };
            acc.1.push(variants_from_status_code);
            acc.2.push(desirable_type_try_from_ident);
            acc
        },
    );
    let (
        unique_status_codes_len,
        unique_status_codes_len_minus_one
     ) = {
        let unique_status_codes_len = unique_status_codes.len();
        if let true = unique_status_codes.is_empty() {
            panic!("{proc_macro_name_ident_stringified} true = unique_status_codes.is_empty()");
        }
        (
            unique_status_codes_len,
            unique_status_codes_len - 1
        )
    };
    let hashmap_attribute_variants = data_enum.variants.iter().fold(
        std::collections::HashMap::<Attribute, Vec<syn::Variant>>::with_capacity(unique_status_codes_len),
        |mut acc, variant|{
            let mut option_attribute = None;
            variant.attrs.iter().for_each(|attr| {
                if let true = attr.path.segments.len() == 1 {
                    if let Ok(named_attribute) =
                        Attribute::try_from(&attr.path.segments[0].ident.to_string())
                    {
                        if let true = option_attribute.is_some() {
                            panic!(
                                "{proc_macro_name_ident_stringified} duplicated attributes are not supported"
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
                panic!("{proc_macro_name_ident_stringified} no supported attribute");
            };
            match acc.get_mut(&attr) {
                Some(i) => {
                    i.push(variant.clone());
                },
                None => {
                    acc.insert(attr, vec![variant.clone()]);
                },
            }
            acc
    });
    let generated_status_code_enums_with_from_impls = {
        let mut generated_status_code_enums_with_from_impls = hashmap_attribute_variants.iter().map(|(attribute, vec_variants)|{
            let status_code_enum_name_stingified = format!("{ident_response_variants_token_stream}{attribute}");
            let status_code_enum_name_token_stream = status_code_enum_name_stingified
            .parse::<proc_macro2::TokenStream>()
            .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {status_code_enum_name_stingified} {}", proc_macro_helpers::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE));
            let status_enum = proc_macro_helpers::error_occurence::generate_with_serialize_deserialize_version::generate_with_serialize_deserialize_version(
                &supported_enum_variant,
                vec_variants.iter().map(|variant|variant.clone()).collect(),
                &with_serialize_deserialize_lower_case,
                &error_occurence_lower_case,
                &vec_lower_case,
                &hashmap_lower_case,
                &key_lower_case,
                &value_lower_case,
                &proc_macro_name_ident_stringified,
                &syn_type_path_stringified,
                generics_len,
                &supports_only_supported_container_stringified,
                &with_serialize_deserialize_camel_case,
                &unnamed_camel_case,
                &status_code_enum_name_token_stream,
                None,
                false,
            );
            let status_enum_from = {
                let variants = vec_variants.iter().map(|variant|{
                    let fields = if let syn::Fields::Named(fields_named) = &variant.fields {
                        fields_named.named.iter().map(|field| {
                            let field_ident = &field.ident.clone().unwrap_or_else(|| panic!("{proc_macro_name_ident_stringified} field_ident is None {}",     proc_macro_helpers::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE));
                            quote::quote! { #field_ident }
                        })
                    }
                    else {
                        panic!("{proc_macro_name_ident_stringified} variant.fields is not a if let syn::Fields::Named");
                    };
                    let fields_cloned =  fields.clone();
                    let variant_ident = &variant.ident;
                    quote::quote! {
                        #status_code_enum_name_token_stream::#variant_ident{ 
                            #(#fields_cloned),*
                        } => Self::#variant_ident{ 
                            #(#fields),*
                        }
                    }
                });
                quote::quote!{
                    impl std::convert::From<#status_code_enum_name_token_stream> for #ident_response_variants_token_stream {
                        fn from(value : #status_code_enum_name_token_stream) -> Self {
                            match value {
                                #(#variants),*
                            }
                        }
                    } 
                }
            };
            quote::quote!{
                #status_enum
                #status_enum_from
            }
        }).collect::<Vec<proc_macro2::TokenStream>>();
        generated_status_code_enums_with_from_impls.push(quote::quote!{
            #[derive(Debug, serde::Serialize, serde::Deserialize)] 
            enum #desirable_type_enum_name {
                DesirableType(#desirable_type_token_stream)
            } 
            impl std::convert::From<#desirable_type_enum_name> for #ident_response_variants_token_stream {
                fn from(value: #desirable_type_enum_name) -> Self {
                    match value { 
                        #desirable_type_enum_name::DesirableType(i) => Self::DesirableType(i) 
                    }
                }
            } 
        });
        generated_status_code_enums_with_from_impls
    };
    let mut is_last_element_found = false;
    let status_code_enums_try_from_handle = unique_status_codes
        .into_iter()
        .enumerate()
        .fold(
            Vec::with_capacity(unique_status_codes_len),
            |mut acc, (index, status_code_attribute)| 
        {
            let status_code_enum_name_stringified = format!("{ident_response_variants_token_stream}{status_code_attribute}");
            let status_code_enum_name_token_stream = status_code_enum_name_stringified
            .parse::<proc_macro2::TokenStream>()
            .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {status_code_enum_name_stringified} {}", proc_macro_helpers::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE));
            let http_status_code_token_stream = status_code_attribute.to_http_status_code_quote();
            match index == unique_status_codes_len_minus_one{
                    true => {
                        is_last_element_found = true;
                        acc.push(quote::quote! {
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
                        acc.push(quote::quote! {
                            else if status_code == #http_status_code_token_stream {
                                match futures::executor::block_on(
                                    response.json::<#status_code_enum_name_token_stream>(),
                                ) {
                                    Ok(value) => Ok(#ident_response_variants_token_stream::from(value)),
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
            acc
        });
        //todo check if len > 0 - otherwise diffrerent syntax
        //todo - check if desirable_type_status_code contains or not in attributes - different logic
    let mut status_code_enums_try_from = vec![quote::quote! {
        if status_code == #desirable_type_status_code_token_stream {
            match futures::executor::block_on(response.json::<#desirable_type_enum_name>()) {
                Ok(value) => Ok(#ident_response_variants_token_stream::from(value)),
                Err(e) => Err(
                    crate::common::api_request_unexpected_error::ApiRequestUnexpectedError::DeserializeBody {
                        reqwest: e,
                        status_code,
                    },
                ),
            }
        }
    }];
    status_code_enums_try_from_handle.into_iter().for_each(|s|{
        status_code_enums_try_from.push(s);
    });
    if let false = is_last_element_found {
        panic!("{proc_macro_name_ident_stringified} false = is_last_element_found");
    }
    let ident_error_named  = format!("{ident}ErrorNamed");
    let ident_error_named_token_stream = ident_error_named
    .parse::<proc_macro2::TokenStream>()
    .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {ident_error_named} {}", proc_macro_helpers::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE));
    let gen = quote::quote! {
        #enum_with_serialize_deserialize_logic
        impl std::convert::From<&#ident_response_variants_token_stream> for http::StatusCode {
            fn from(value: &#ident_response_variants_token_stream) -> Self {
                match value {
                    #ident_response_variants_token_stream :: DesirableType(_) => http :: StatusCode :: OK,
                    #(#variants_from_status_code),*
                }
            }
        }
        #(#generated_status_code_enums_with_from_impls)*
        impl std::convert::TryFrom<reqwest::Response> for #ident_response_variants_token_stream {
            type Error = crate::common::api_request_unexpected_error::ApiRequestUnexpectedError;
            fn try_from(response: reqwest::Response) -> Result<Self, Self::Error> {
                let status_code = response.status();
                #(#status_code_enums_try_from)*
            }
        }
        impl TryFrom<#ident_response_variants_token_stream> for #desirable_type_token_stream
        {
            type Error = #try_error_ident_token_stream;
            fn try_from(
                value: #ident_response_variants_token_stream,
            ) -> Result<Self, Self::Error> {
                match value {
                    #ident_response_variants_token_stream :: DesirableType(i) => Ok(i),
                    #(#desirable_type_try_from_ident),*
                }
            }
        }
        //
        #[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
        pub enum #ident_error_named_token_stream<'a> {//todo
            ExpectedType {
                #[eo_display_with_serialize_deserialize]
                get: #try_error_ident_token_stream,
                code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
            },
            UnexpectedStatusCode { 
                #[eo_display]
                status_code: http::StatusCode,
                code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
            },
            DeserializeResponse {
                #[eo_display_foreign_type]
                reqwest: reqwest::Error,
                #[eo_display]
                status_code: http::StatusCode,
                code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
            },
            Reqwest {
                #[eo_display_foreign_type]
                reqwest: reqwest::Error,
                code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
            },
        }
        async fn get_extraction_logic<'a>(
            future: impl std::future::Future<Output = Result<reqwest::Response, reqwest::Error>>,
        ) -> Result<#desirable_type_token_stream, #ident_error_named_token_stream<'a>>
        {
            match future.await {
                Ok(response) => match TryGetResponseVariants::try_from(response) {
                    Ok(variants) => match #desirable_type_token_stream::try_from(variants)
                    {
                        Ok(value) => Ok(value),
                        Err(e) => Err(#ident_error_named_token_stream::ExpectedType {
                            get: e,
                            code_occurence: crate::code_occurence_tufa_common!(),
                        }),
                    },
                    Err(e) => match e {//todo impl from?
                        crate::common::api_request_unexpected_error::ApiRequestUnexpectedError::StatusCode { status_code } => Err(
                            #ident_error_named_token_stream::UnexpectedStatusCode {
                                status_code,
                                code_occurence: crate::code_occurence_tufa_common!()
                            }
                        ),
                        crate::common::api_request_unexpected_error::ApiRequestUnexpectedError::DeserializeBody {
                            reqwest,
                            status_code
                        } => Err(
                            #ident_error_named_token_stream::DeserializeResponse {
                                reqwest,
                                status_code,
                                code_occurence: crate::code_occurence_tufa_common!()
                            }
                        ),
                    },
                },
                Err(e) => Err(#ident_error_named_token_stream::Reqwest {
                    reqwest: e,
                    code_occurence: crate::code_occurence_tufa_common!(),
                }),
            }
        }
        //
    };
    // if ident == "" {
    //   println!("{gen}");
    // }
    gen.into()
}