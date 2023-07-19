static STATUS_CODES_CHECKER: &str = "StatusCodesChecker";
static PATH: &str = "type_variants_from_reqwest_response";
static RESPONSE_VARIANTS: &str = "ResponseVariants";

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

fn get_macro_attribute<'a>(
    attrs: &'a [syn::Attribute],
    attribute_path: std::string::String,
    ident: &syn::Ident,
    macro_name: &str
) -> &'a syn::Attribute {
    let option_attribute = attrs.iter().find(|attr| {
        attribute_path == {
            let mut stringified_path = quote::ToTokens::to_token_stream(&attr.path).to_string();
            stringified_path.retain(|c| !c.is_whitespace());
            stringified_path
        }
    });
    if let Some(attribute) = option_attribute {
        attribute
    }
    else {
        panic!("{macro_name} {ident}no {attribute_path}");
    }
}
fn get_vec_enum_paths(
    attribute: &syn::Attribute,
    ident: &syn::Ident,
    macro_name: &str
) -> Vec<std::string::String> {
    let mut stringified_tokens = quote::ToTokens::to_token_stream(&attribute.tokens).to_string();
    stringified_tokens.retain(|c| !c.is_whitespace());
    match stringified_tokens.len() > 3 {
        true => {
            let mut chars = stringified_tokens.chars();
            match (chars.next(), chars.last()) {
                (None, None) => panic!("{macro_name} {ident} no first and last token attribute"),
                (None, Some(_)) => panic!("{macro_name} {ident} no first token attribute"),
                (Some(_), None) => panic!("{macro_name} {ident} no last token attribute"),
                (Some(first), Some(last)) => match (first == '(', last == ')') {
                    (true, true) => {
                        match stringified_tokens.get(1..(stringified_tokens.len()-1)) {
                            Some(inner_tokens_str) => {
                                inner_tokens_str.split(',').map(|str|{str.to_string()}).collect::<Vec<std::string::String>>()
                            },
                            None => panic!("{macro_name} {ident} cannot get inner_token"),
                        }
                    },
                    (true, false) => panic!("{macro_name} {ident} last token attribute is not )"),
                    (false, true) => panic!("{macro_name} {ident} first token attribute is not ("),
                    (false, false) => panic!("{macro_name} {ident} first token attribute is not ( and last token attribute is not )"),
                },
            }
        }
        false => panic!("{macro_name} {ident} stringified_tokens.len() > 3 == false"),
    }
}
fn generate_from_logic(
    ident: &syn::Ident,
    macro_name: &str,
    ident_response_variants_stringified: &std::string::String,
    variants: &syn::punctuated::Punctuated<syn::Variant, syn::token::Comma>
) -> proc_macro2::TokenStream {
    let ident_with_serialize_deserialize_stringified = format!("{ident}{}", proc_macro_helpers::error_occurence::hardcode::with_serialize_deserialize_camel_case());
    let ident_with_serialize_deserialize_token_stream = ident_with_serialize_deserialize_stringified
    .parse::<proc_macro2::TokenStream>()
    .unwrap_or_else(|_| {
        panic!("{macro_name} {ident} {}", proc_macro_helpers::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE)
    });
    let enum_path_token_stream = ident_response_variants_stringified
        .parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| {
            panic!("{macro_name} {ident} {ident_response_variants_stringified} {}", proc_macro_helpers::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE)
        });
    let variants = {
        variants.iter().map(|variant| {
            let variant_ident = &variant.ident;
            match &variant.fields {
                syn::Fields::Named(fields_named) => {
                    let fields_generated = fields_named.named.iter().map(|field|{
                        field.ident.clone().unwrap_or_else(|| {
                            panic!("{macro_name} {ident} {ident_response_variants_stringified} field ident is None")
                        })
                    });
                    let fields_generated_cloned = fields_generated.clone();
                    quote::quote! {
                        #ident_with_serialize_deserialize_token_stream::#variant_ident { #(#fields_generated),* } => Self::#variant_ident { #(#fields_generated_cloned),* }
                    }
                }
                syn::Fields::Unnamed(fields_unnamed) => {
                    if let false = fields_unnamed.unnamed.len() == 1 {
                        panic!("{macro_name} {ident} fields_unnamed.unnamed.len() != 1");
                    }
                    quote::quote! {
                        #ident_with_serialize_deserialize_token_stream::#variant_ident(i) => Self::#variant_ident(i)
                    }
                }
                syn::Fields::Unit => panic!(
                    "{macro_name} {ident} works only with syn::Fields::Named and syn::Fields::Unnamed"
                ),
            }
        })
    };
    let gen = quote::quote! {
        impl<'a> std::convert::From<#ident<'a>>
            for #enum_path_token_stream
        {
            fn from(
                val: #ident<'a>,
            ) -> Self {
                match val.into_serialize_deserialize_version() {
                    #(#variants),*
                }
            }
        }
    };
    // if ident_response_variants_stringified == "" {
    //     println!("{gen}");
    // }
    gen
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
    let ident_response_variants_stringified = format!("{ident}{RESPONSE_VARIANTS}");
    let ident_response_variants_token_stream = ident_response_variants_stringified
    .parse::<proc_macro2::TokenStream>()
    .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {ident_response_variants_stringified} {}", proc_macro_helpers::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE));
    let attribute = get_macro_attribute(
        &ast.attrs,
        format!("{PATH}::type_variants_from_reqwest_response_attribute"),
        ident,
        macro_name
    );
    let (
        desirable_type_token_stream,
        desirable_type_status_code_token_stream,
        desirable_type_enum_name,
        desirable_type_attribute
    ) = {
        let mut stringified_tokens =
            quote::ToTokens::to_token_stream(&attribute.tokens).to_string();
        stringified_tokens.retain(|c| !c.is_whitespace());
        match stringified_tokens.len() > 3 {
            true => {
                let mut chars = stringified_tokens.chars();
                match (chars.next(), chars.last()) {
                        (None, None) => panic!("{macro_name} {ident} no first and last token attribute"),
                        (None, Some(_)) => panic!("{macro_name} {ident} no first token attribute"),
                        (Some(_), None) => panic!("{macro_name} {ident} no last token attribute"),
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
                                                    },
                                                    attribute
                                                )
                                            },
                                        }
                                    },
                                    None => panic!("{macro_name} {ident} cannot get inner_token"),
                                }
                            },
                            (true, false) => panic!("{macro_name} {ident} last token attribute is not )"),
                            (false, true) => panic!("{macro_name} {ident} first token attribute is not ("),
                            (false, false) => panic!("{macro_name} {ident} first token attribute is not ( and last token attribute is not )"),
                        },
                    }
            }
            false => panic!("{macro_name} {ident} {stringified_tokens}.len() > 3 == false"),
        }
    };
    let response_without_body = desirable_type_token_stream.to_string() == "()";
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
    let variants_len = data_enum.variants.len();
    let enum_status_codes_checker_name_stringified = format!("{ident}{STATUS_CODES_CHECKER}");
    let enum_status_codes_checker_name_token_stream = enum_status_codes_checker_name_stringified
    .parse::<proc_macro2::TokenStream>()
    .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {enum_status_codes_checker_name_stringified} {}", proc_macro_helpers::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE));
    let enum_status_codes_checker_variants = data_enum.variants.iter().fold(
        Vec::with_capacity(variants_len),
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
            let check_variant_ident_stringified = format!("{}{}", variant.ident, attr);
            acc.push(
                check_variant_ident_stringified
                .parse::<proc_macro2::TokenStream>()
                .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {check_variant_ident_stringified} {}", proc_macro_helpers::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
            );
            acc
        }
    );
    let try_error_ident_stringified = format!("{ident}{}", proc_macro_helpers::error_occurence::hardcode::with_serialize_deserialize_camel_case());
    let try_error_ident_token_stream = try_error_ident_stringified
    .parse::<proc_macro2::TokenStream>()
    .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {try_error_ident_stringified} {}", proc_macro_helpers::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE));
    let (
        unique_status_codes, 
        variants_from_status_code,
        desirable_type_try_from_ident,
    ) = data_enum.variants.iter().fold(
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
            if !acc.0.contains(&attr) {
                acc.0.push(attr)
            }
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
        std::collections::HashMap::<Attribute, Vec<&syn::Variant>>::with_capacity(unique_status_codes_len),
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
                    i.push(variant);
                },
                None => {
                    acc.insert(attr, vec![variant]);
                },
            }
            acc
    });
    let desirable_type_name_token_stream = quote::quote!{DesirableType};
    let generated_status_code_enums_with_from_impls = {
        let mut is_desirable_type_detected = false;
        let mut generated_status_code_enums_with_from_impls = hashmap_attribute_variants.iter().map(|(attribute, vec_variants)|{
            let status_code_enum_name_stingified = format!("{ident_response_variants_token_stream}{attribute}");
            let status_code_enum_name_token_stream = status_code_enum_name_stingified
            .parse::<proc_macro2::TokenStream>()
            .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {status_code_enum_name_stingified} {}", proc_macro_helpers::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE));
            let (
                optional_additional_named_variant, 
                additional_from_variant
            ) = match attribute == &desirable_type_attribute {
                true => {
                    is_desirable_type_detected = true;
                    (
                        Some(quote::quote! {
                            #desirable_type_name_token_stream(#desirable_type_token_stream)
                        }),
                        quote::quote!{
                            #desirable_type_enum_name::#desirable_type_name_token_stream(i) => Self::#desirable_type_name_token_stream(i),
                        }
                    )
                },
                false => (
                    None,
                    proc_macro2::TokenStream::new()
                )
            };
            let status_enum = proc_macro_helpers::error_occurence::generate_with_serialize_deserialize_version::generate_with_serialize_deserialize_version(
                &supported_enum_variant,
                vec_variants,
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
                optional_additional_named_variant,
                false,
                false
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
                                #additional_from_variant
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
        if !is_desirable_type_detected {
            if let false = response_without_body {
                generated_status_code_enums_with_from_impls.push(quote::quote!{
                    #[derive(Debug, serde::Serialize, serde::Deserialize)] 
                    enum #desirable_type_enum_name {
                        #desirable_type_name_token_stream(#desirable_type_token_stream)
                    } 
                    impl std::convert::From<#desirable_type_enum_name> for #ident_response_variants_token_stream {
                        fn from(value: #desirable_type_enum_name) -> Self {
                            match value { 
                                #desirable_type_enum_name::#desirable_type_name_token_stream(i) => Self::#desirable_type_name_token_stream(i) 
                            }
                        }
                    }    
                });
            }
        }
        generated_status_code_enums_with_from_impls
    };
    let mut is_last_element_found = false;
    let api_request_unexpected_error_module_path_token_stream = quote::quote! { crate::common::api_request_unexpected_error };
    let api_request_unexpected_error_path_token_stream = quote::quote! { #api_request_unexpected_error_module_path_token_stream::ApiRequestUnexpectedError };
    let status_code_enums_try_from = {
        let desirable_type_status_code_case_token_stream = match response_without_body {
            true => quote::quote! {
                Ok(#ident_response_variants_token_stream::#desirable_type_name_token_stream(()))
            },
            false => quote::quote! {
                match response.text().await {
                    Ok(response_text) => match serde_json::from_str::<#desirable_type_enum_name>(&response_text){
                        Ok(value) => Ok(#ident_response_variants_token_stream::from(value)), 
                        Err(e) => Err(
                            #api_request_unexpected_error_path_token_stream::DeserializeBody{ 
                                serde: e,
                                status_code,
                                headers,response_text
                            }
                        ),
                    },
                    Err(e) => Err(
                        #api_request_unexpected_error_path_token_stream::FailedToGetResponseText {
                            reqwest: e,
                            status_code,
                            headers,
                        }
                    ),
                }
            },
        };
        let mut status_code_enums_try_from_variants = Vec::with_capacity(unique_status_codes_len + 1);
        status_code_enums_try_from_variants.push(quote::quote! {
            if status_code == #desirable_type_status_code_token_stream {
                #desirable_type_status_code_case_token_stream
            }
        });
        unique_status_codes
        .into_iter()
        .enumerate()
        .for_each(
            |(index, status_code_attribute)| 
        {
            let status_code_enum_name_stringified = format!("{ident_response_variants_token_stream}{status_code_attribute}");
            let status_code_enum_name_token_stream = status_code_enum_name_stringified
            .parse::<proc_macro2::TokenStream>()
            .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {status_code_enum_name_stringified} {}", proc_macro_helpers::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE));
            let http_status_code_token_stream = status_code_attribute.to_http_status_code_quote();
            match index == unique_status_codes_len_minus_one{
                    true => {
                        is_last_element_found = true;
                        status_code_enums_try_from_variants.push(quote::quote! {
                            else {
                                match response.text().await {
                                    Ok(response_text) => Err(
                                        #api_request_unexpected_error_path_token_stream::StatusCode {
                                            status_code,
                                            headers,
                                            response_text_result: #api_request_unexpected_error_module_path_token_stream::ResponseTextResult::ResponseText(response_text)
                                        },
                                    ),
                                    Err(e) => Err(
                                        #api_request_unexpected_error_path_token_stream::StatusCode {
                                            status_code,
                                            headers,
                                            response_text_result: #api_request_unexpected_error_module_path_token_stream::ResponseTextResult::ReqwestError(e),
                                        },
                                    ),
                                }
                            }
                        });
                    },
                    false => {
                        if let false = desirable_type_attribute == status_code_attribute {
                            status_code_enums_try_from_variants.push(quote::quote! {
                                else if status_code == #http_status_code_token_stream {
                                    match response.text().await {
                                        Ok(response_text) => match serde_json::from_str::<#status_code_enum_name_token_stream>(&response_text){
                                            Ok(value) => Ok(#ident_response_variants_token_stream::from(value)), 
                                            Err(e) => Err(
                                                #api_request_unexpected_error_path_token_stream::DeserializeBody{ 
                                                    serde: e,
                                                    status_code,
                                                    headers,response_text
                                                }
                                            ),
                                        },
                                        Err(e) => Err(
                                            #api_request_unexpected_error_path_token_stream::FailedToGetResponseText {
                                                reqwest: e,
                                                status_code,
                                                headers,
                                            }
                                        ),
                                    }
                                }
                            });
                        }
                    },
                }
        });
        status_code_enums_try_from_variants
    };
    if let false = is_last_element_found {
        panic!("{proc_macro_name_ident_stringified} false = is_last_element_found");
    }
    let ident_request_error  = format!("{ident}RequestError");
    let ident_request_error_token_stream = ident_request_error
    .parse::<proc_macro2::TokenStream>()
    .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {ident_request_error} {}", proc_macro_helpers::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE));
    let enum_with_serialize_deserialize_logic_token_stream = proc_macro_helpers::error_occurence::generate_with_serialize_deserialize_version::generate_with_serialize_deserialize_version(
        &supported_enum_variant,
        &data_enum.variants.iter().collect(),
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
            #desirable_type_name_token_stream(#desirable_type_token_stream)
        }),
        false,
        true
    );
    let from_logic_token_stream = generate_from_logic(
        ident,
        macro_name,
        &ident_response_variants_stringified,
        &data_enum.variants
    );
    let impl_from_ident_response_variants_token_stream_for_actix_web_http_response_logic_token_stream = quote::quote! {
        impl From<#ident_response_variants_token_stream> for actix_web::HttpResponse {
            fn from(val: #ident_response_variants_token_stream) -> Self {
                let mut actix_web_http_response = actix_web::HttpResponseBuilder::new((&val).into());
                actix_web_http_response.json(actix_web::web::Json(val))
            }
        }
    };
    let impl_std_convert_from_ident_response_variants_token_stream_for_http_status_code_logic_token_stream = quote::quote! {
        impl std::convert::From<&#ident_response_variants_token_stream> for http::StatusCode {
            fn from(value: &#ident_response_variants_token_stream) -> Self {
                match value {
                    #ident_response_variants_token_stream::#desirable_type_name_token_stream(_) => #desirable_type_status_code_token_stream,
                    #(#variants_from_status_code),*
                }
            }
        }
    };
    let generated_status_code_enums_with_from_impls_logic_token_stream = quote::quote! {
        #(#generated_status_code_enums_with_from_impls)*
    };
    let try_from_response_logic_token_stream = quote::quote! {
        async fn try_from_response(response: reqwest::Response) -> Result<#ident_response_variants_token_stream, #api_request_unexpected_error_path_token_stream> {
            let status_code = response.status();
            let headers = response.headers().clone();
            #(#status_code_enums_try_from)*
        }
    };
    let impl_try_from_ident_response_variants_token_stream_for_desirable_type_logic_token_stream = match response_without_body {
        true => quote::quote! {},
        false => quote::quote! {
            impl TryFrom<#ident_response_variants_token_stream> for #desirable_type_token_stream
            {
                type Error = #try_error_ident_token_stream;
                fn try_from(
                    value: #ident_response_variants_token_stream,
                ) -> Result<Self, Self::Error> {
                    match value {
                        #ident_response_variants_token_stream::#desirable_type_name_token_stream(i) => Ok(i),
                        #(#desirable_type_try_from_ident),*
                    }
                }
            }
        },
    };
    let ident_request_error_logic_token_stream = quote::quote! {
        #[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
        pub enum #ident_request_error_token_stream<'a> {
            ExpectedType {
                #[eo_display_with_serialize_deserialize]
                expected_type: #try_error_ident_token_stream,
                code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
            },
            UnexpectedStatusCode {
                #[eo_display]
                status_code: http::StatusCode,
                #[eo_display_foreign_type]
                headers: reqwest::header::HeaderMap,
                #[eo_display_foreign_type]
                response_text_result: #api_request_unexpected_error_module_path_token_stream::ResponseTextResult,
                code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
            },
            FailedToGetResponseText {
                #[eo_display_foreign_type]
                reqwest: reqwest::Error,
                #[eo_display]
                status_code: http::StatusCode,
                #[eo_display_foreign_type]
                headers: reqwest::header::HeaderMap,
                code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
            },
            DeserializeResponse {
                #[eo_display]
                serde: serde_json::Error,
                #[eo_display]
                status_code: http::StatusCode,
                #[eo_display_foreign_type]
                headers: reqwest::header::HeaderMap,
                #[eo_display_with_serialize_deserialize]
                response_text: std::string::String,
                code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
            },
            Reqwest {
                #[eo_display_foreign_type]
                reqwest: reqwest::Error,
                code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
            },
        }
    };
    let extraction_logic_token_stream = {
        let response_without_body_logic_token_stream = match response_without_body {
            true => quote::quote! {
                Ok(_variants) => Ok(())
            },
            false => quote::quote! {
                Ok(variants) => match #desirable_type_token_stream::try_from(variants)
                {
                    Ok(value) => Ok(value),
                    Err(e) => Err(#ident_request_error_token_stream::ExpectedType {
                        expected_type: e,
                        code_occurence: crate::code_occurence_tufa_common!(),
                    }),
                }
            },
        };
        quote::quote! {
            async fn tvfrr_extraction_logic<'a>(
                future: impl std::future::Future<Output = Result<reqwest::Response, reqwest::Error>>,
            ) -> Result<#desirable_type_token_stream, #ident_request_error_token_stream<'a>>
            {
                match future.await {
                Ok(response) => match try_from_response(response).await {
                    #response_without_body_logic_token_stream,
                    Err(e) => match e {
                        #api_request_unexpected_error_path_token_stream::StatusCode { 
                            status_code,
                            headers,
                            response_text_result,
                        } => Err(
                            #ident_request_error_token_stream::UnexpectedStatusCode {
                                status_code,
                                headers,
                                response_text_result,
                                code_occurence: crate::code_occurence_tufa_common!()
                            }
                        ),
                        #api_request_unexpected_error_path_token_stream::FailedToGetResponseText {
                            reqwest, 
                            status_code, 
                            headers 
                        } => Err(
                            #ident_request_error_token_stream::FailedToGetResponseText {
                                reqwest,
                                status_code,
                                headers,
                                code_occurence: crate::code_occurence_tufa_common!()
                            }
                        ),
                        #api_request_unexpected_error_path_token_stream::DeserializeBody {
                            serde, 
                            status_code,
                            headers,
                            response_text,
                        } => Err(
                            #ident_request_error_token_stream::DeserializeResponse {
                                serde, 
                                status_code,
                                headers,
                                response_text,
                                code_occurence: crate::code_occurence_tufa_common!()
                            }
                        ),
                    },
                },
                Err(e) => Err(#ident_request_error_token_stream::Reqwest {
                    reqwest: e,
                    code_occurence: crate::code_occurence_tufa_common!(),
                }),
            }
        }
    }
    };
    let enum_status_codes_checker_name_logic_token_stream = quote::quote! {
        pub enum #enum_status_codes_checker_name_token_stream {
            #(#enum_status_codes_checker_variants),*
        }
    };
    let gen = quote::quote! {
        #enum_with_serialize_deserialize_logic_token_stream
        #from_logic_token_stream
        #impl_from_ident_response_variants_token_stream_for_actix_web_http_response_logic_token_stream
        #impl_std_convert_from_ident_response_variants_token_stream_for_http_status_code_logic_token_stream
        #generated_status_code_enums_with_from_impls_logic_token_stream
        #try_from_response_logic_token_stream
        #impl_try_from_ident_response_variants_token_stream_for_desirable_type_logic_token_stream
        #ident_request_error_logic_token_stream
        #extraction_logic_token_stream
        #enum_status_codes_checker_name_logic_token_stream
    };
    // if ident == "" {
    //   println!("{gen}");
    // }
    gen.into()
}

#[proc_macro_attribute]
pub fn type_variants_from_reqwest_response_attribute(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    item
}

#[proc_macro_derive(
    TypeVariantsFromReqwestResponseFromChecker,
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
pub fn type_variants_from_reqwest_response_from_checker(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_helpers::panic_location::panic_location(); //panic_location function from https://github.com/kuqmua/proc_macro_helpers
    let macro_name = "TypeVariantsFromReqwestResponseFromChecker";
    let ast: syn::DeriveInput = syn::parse_macro_input!(input as syn::DeriveInput);
    let ident = &ast.ident;
    let proc_macro_name_ident_stringified = format!("{macro_name} {ident}");
    let enum_status_codes_checker_stringified = format!("{ident}{STATUS_CODES_CHECKER}");
    let enum_status_codes_checker_name_token_stream = enum_status_codes_checker_stringified
    .parse::<proc_macro2::TokenStream>()
    .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {enum_status_codes_checker_stringified} {}", proc_macro_helpers::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE));
    let variants = if let syn::Data::Enum(data_enum) = ast.data {
        data_enum.variants
    } else {
        panic!("{macro_name} does work only on enums!");
    };
    let enum_status_codes_checker_variants = variants.iter().map(|variant|{
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
        let check_variant_ident_stringified = format!("{}{}", variant.ident, attr);
        check_variant_ident_stringified
        .parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {check_variant_ident_stringified} {}", proc_macro_helpers::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    });
    let attribute = get_macro_attribute(
        &ast.attrs,
        format!("{PATH}::type_variants_from_reqwest_response_from_checker_paths"),
        ident,
        macro_name
    );
    let vec_enum_paths = get_vec_enum_paths(
        attribute,
        ident,
        macro_name,
    );
    let variants_len = variants.len();
    let variants_from_status_code = variants.iter().fold(
         Vec::with_capacity(variants_len),
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
            let http_status_code_token_stream = if let Some(attr) = option_attribute {
                attr.to_http_status_code_quote()
            } else {
                panic!("{proc_macro_name_ident_stringified} no supported attribute");
            };
            let variant_ident = &variant.ident;
            acc.push(match &variant.fields {
                syn::Fields::Named(fields_named) => {
                    let fields_named_named_len = fields_named.named.len();
                    let fields_token_stream_variants_from_status_code = fields_named.named.iter().fold(
                        Vec::with_capacity(fields_named_named_len),
                        |mut acc, field| {
                            let field_ident = &field.clone().ident.unwrap_or_else(|| {
                                panic!("{proc_macro_name_ident_stringified} named field ident is None");
                            });
                            acc.push(quote::quote! { #field_ident: _ });
                            acc
                        }
                    );
                    quote::quote! {
                        #ident::#variant_ident {
                            #(#fields_token_stream_variants_from_status_code),*
                        } => #http_status_code_token_stream,
                    }
                }
                syn::Fields::Unnamed(fields_unnamed) => {
                    let fields_token_stream = if let true = fields_unnamed.unnamed.len() == 1 {
                        quote::quote! { _ }
                    }
                    else {
                        panic!("{proc_macro_name_ident_stringified} fields_unnamed.unnamed.len() != 1");                       
                    };
                    quote::quote! {
                        #ident::#variant_ident(#fields_token_stream) => #http_status_code_token_stream
                    }
                }
                syn::Fields::Unit => {
                    panic!("{proc_macro_name_ident_stringified} syn::Data is not a syn::Data::Enum")
                }
            });
            acc
        },
    );
    let enum_status_codes_checker_from_impls = vec_enum_paths.iter().map(|enum_path| {
        let enum_path_stringified = format!("{enum_path}{STATUS_CODES_CHECKER}");
        let enum_path_token_stream = enum_path_stringified
            .parse::<proc_macro2::TokenStream>()
            .unwrap_or_else(|_| {
                panic!("{macro_name} {ident} {enum_path_stringified} {}", proc_macro_helpers::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE)
            });
        let enum_status_codes_checker_from_impls_variants = variants.iter().map(|variant| {
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
            let check_variant_ident_stringified = format!("{}{}", variant.ident, attr);
            let check_variant_ident_token_stream = check_variant_ident_stringified
            .parse::<proc_macro2::TokenStream>()
            .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {check_variant_ident_stringified} {}", proc_macro_helpers::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE));
            match &variant.fields {
                syn::Fields::Named(_fields_named) => {
                    quote::quote! {
                        #enum_status_codes_checker_name_token_stream::#check_variant_ident_token_stream => #enum_path_token_stream::#check_variant_ident_token_stream
                    }
                }
                syn::Fields::Unnamed(fields_unnamed) => {
                    if let false = fields_unnamed.unnamed.len() == 1 {
                        panic!("{macro_name} {ident} fields_unnamed.unnamed.len() != 1");
                    }
                    quote::quote! {
                        #enum_status_codes_checker_name_token_stream::#check_variant_ident_token_stream => #enum_path_token_stream::#check_variant_ident_token_stream
                    }
                }
                syn::Fields::Unit => panic!(
                    "{macro_name} {ident} works only with syn::Fields::Named and syn::Fields::Unnamed"
                ),
            }
        });
        //
        let variant_gen = quote::quote! {
            impl std::convert::From<#enum_status_codes_checker_name_token_stream>
                for #enum_path_token_stream
            {
                fn from(
                    val: #enum_status_codes_checker_name_token_stream,
                ) -> Self {
                    match val {
                        #(#enum_status_codes_checker_from_impls_variants),*
                    }
                }
            }
        };
        // if enum_path == "" {
        //     println!("{variant_gen}");
        // }
        variant_gen
    });
    let vec_enum_paths_len =  vec_enum_paths.len();
    let (
        generated_response_variants,
        generated_with_serialize_deserialize
     ) = vec_enum_paths.iter().fold(
        (
            Vec::with_capacity(vec_enum_paths_len),
            Vec::with_capacity(vec_enum_paths_len),
        ),
        |mut acc, enum_path| {
            acc.0.push(generate_from_logic(
                ident,
                macro_name,
                &format!("{enum_path}{RESPONSE_VARIANTS}"),
                &variants
            ));
            acc.1.push(generate_from_logic(
                ident,
                macro_name,
                &format!("{enum_path}{}", proc_macro_helpers::error_occurence::hardcode::with_serialize_deserialize_camel_case()),
                &variants
            ));
            acc
        }
    );
    let gen = quote::quote! {
        #(#generated_response_variants)*
        #(#generated_with_serialize_deserialize)*
        impl<'a> From<&#ident<'a>> for http::StatusCode {
            fn from(val: &#ident<'a>) -> Self {
                match &val {
                    #(#variants_from_status_code)*
                }
            }
        }
        #[allow(clippy::enum_variant_names)]
        enum #enum_status_codes_checker_name_token_stream {
            #(#enum_status_codes_checker_variants),*
        }
        #(#enum_status_codes_checker_from_impls)*
    };
    // if ident == "" {
    //     println!("{gen}");
    // }
    gen.into()
}

#[proc_macro_attribute]
pub fn type_variants_from_reqwest_response_from_checker_paths(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    item
}