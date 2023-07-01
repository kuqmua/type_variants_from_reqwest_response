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
    // let tvfrr_100 = "tvfrr_100";
    // let tvfrr_101 = "tvfrr_101";
    // let tvfrr_102 = "tvfrr_102";
    // let tvfrr_200 = "tvfrr_200";
    // let tvfrr_201 = "tvfrr_201";
    // let tvfrr_202 = "tvfrr_202";
    // let tvfrr_203 = "tvfrr_203";
    // let tvfrr_204 = "tvfrr_204";
    // let tvfrr_205 = "tvfrr_205";
    // let tvfrr_206 = "tvfrr_206";
    // let tvfrr_207 = "tvfrr_207";
    // let tvfrr_208 = "tvfrr_208";
    // let tvfrr_226 = "tvfrr_226";
    // let tvfrr_300 = "tvfrr_300";
    // let tvfrr_301 = "tvfrr_301";
    // let tvfrr_302 = "tvfrr_302";
    // let tvfrr_303 = "tvfrr_303";
    // let tvfrr_304 = "tvfrr_304";
    // let tvfrr_305 = "tvfrr_305";
    // let tvfrr_307 = "tvfrr_307";
    // let tvfrr_308 = "tvfrr_308";
    // let tvfrr_400 = "tvfrr_400";
    // let tvfrr_401 = "tvfrr_401";
    // let tvfrr_402 = "tvfrr_402";
    // let tvfrr_403 = "tvfrr_403";
    // let tvfrr_404 = "tvfrr_404";
    // let tvfrr_405 = "tvfrr_405";
    // let tvfrr_406 = "tvfrr_406";
    // let tvfrr_407 = "tvfrr_407";
    // let tvfrr_408 = "tvfrr_408";
    // let tvfrr_409 = "tvfrr_409";
    // let tvfrr_410 = "tvfrr_410";
    // let tvfrr_411 = "tvfrr_411";
    // let tvfrr_412 = "tvfrr_412";
    // let tvfrr_413 = "tvfrr_413";
    // let tvfrr_414 = "tvfrr_414";
    // let tvfrr_415 = "tvfrr_415";
    // let tvfrr_416 = "tvfrr_416";
    // let tvfrr_417 = "tvfrr_417";
    // let tvfrr_418 = "tvfrr_418";
    // let tvfrr_421 = "tvfrr_421";
    // let tvfrr_422 = "tvfrr_422";
    // let tvfrr_423 = "tvfrr_423";
    // let tvfrr_424 = "tvfrr_424";
    // let tvfrr_426 = "tvfrr_426";
    // let tvfrr_428 = "tvfrr_428";
    // let tvfrr_429 = "tvfrr_429";
    // let tvfrr_431 = "tvfrr_431";
    // let tvfrr_451 = "tvfrr_451";
    // let tvfrr_500 = "tvfrr_500";
    // let tvfrr_501 = "tvfrr_501";
    // let tvfrr_502 = "tvfrr_502";
    // let tvfrr_503 = "tvfrr_503";
    // let tvfrr_504 = "tvfrr_504";
    // let tvfrr_505 = "tvfrr_505";
    // let tvfrr_506 = "tvfrr_506";
    // let tvfrr_507 = "tvfrr_507";
    // let tvfrr_508 = "tvfrr_508";
    // let tvfrr_510 = "tvfrr_510";
    // let tvfrr_511 = "tvfrr_511";

    let ast: syn::DeriveInput = syn::parse(input).unwrap_or_else(|_| {
        panic!("{macro_name} let ast: syn::DeriveInput = syn::parse(input) failed")
    });
    let ident = &ast.ident;
    let data_enum = if let syn::Data::Enum(data_enum) = ast.data {
        data_enum
    } else {
        panic!("{macro_name} {ident} syn::Data is not a syn::Data::Enum");
    };
    let variants_len = data_enum.variants.len();
    let (unique_status_codes, variants_with_status_code) = data_enum.variants.into_iter().fold(
        (
            Vec::with_capacity(variants_len),
            Vec::with_capacity(variants_len),
        ),
        |mut acc, variant| {
            let mut option_attribute = None;
            variant.attrs.iter().for_each(|attr| {
                if let true = attr.path.segments.len() == 1 {
                    if let Ok(named_attribute) = Attribute::try_from(&attr.path.segments[0].ident) {
                        if let true = option_attribute.is_some() {
                            panic!("{macro_name} {ident} duplicated attributes are not supported");
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
            acc.1.push((attr, variant));
            acc
        },
    );
    // println!("unique_status_codes {unique_status_codes:#?}");
    unique_status_codes
        .iter()
        .for_each(|status_code_attribute| {
            let status_code_enum_name = format!("{ident}{status_code_attribute}");
            println!("{status_code_enum_name}");
            let status_code_variants_vec = variants_with_status_code.iter().fold(
                Vec::with_capacity(variants_len),
                |mut acc, (attribute, variant)| {
                    if let true = status_code_attribute == attribute {
                        acc.push(variant);
                    }
                    acc
                },
            );
            println!("{}", status_code_variants_vec.len());
            // (
            //     quote::quote! {
            //         // #[derive(Debug, serde :: Serialize, serde :: Deserialize)]
            //         // enum GetHttpResponseVariantsInternalServerError {
            //         //     Configuration {
            //         //         configuration_box_dyn_error: std::string::String,
            //         //         code_occurence: crate::common::code_occurence::CodeOccurenceWithSerializeDeserialize,
            //         //     },
            //         //     Database {
            //         //         box_dyn_database_error: std::string::String,
            //         //         code_occurence: crate::common::code_occurence::CodeOccurenceWithSerializeDeserialize,
            //         //     },
            //         // }
            //         // impl std::convert::From<GetHttpResponseVariantsInternalServerError> for GetHttpResponseVariants {
            //         //     fn from(val: GetHttpResponseVariantsInternalServerError) -> Self {
            //         //         match val {
            //         //             GetHttpResponseVariantsInternalServerError::Configuration {
            //         //                 configuration_box_dyn_error,
            //         //                 code_occurence,
            //         //             } => Self::Configuration {
            //         //                 configuration_box_dyn_error,
            //         //                 code_occurence,
            //         //             },
            //         //         }
            //         //     }
            //         // }
            //     },
            //     quote::quote! {},
            // )
        });
    let gen = quote::quote! {


    // impl std::convert::TryFrom<reqwest::Response> for GetHttpResponseVariants {
    //     type Error = GetHttpResponseVariantsTryFromReqwestResponseVariant;
    //     fn try_from(response: reqwest::Response) -> Result<Self, Self::Error> {
    //         let status_code = response.status();
    //         if status_code == http::StatusCode::OK {
    //             match futures::executor::block_on(response.json::<GetHttpResponseVariantsOk>()) {
    //                 Ok(value) => Ok(GetHttpResponseVariants::from(value)),
    //                 Err(e) => Err(
    //                     GetHttpResponseVariantsTryFromReqwestResponseVariant::DeserializeResponse {
    //                         reqwest: e,
    //                         status_code,
    //                     },
    //                 ),
    //             }
    //         } else if status_code == http::StatusCode::BAD_REQUEST {
    //             match futures::executor::block_on(response.json::<GetHttpResponseVariantsBadRequest>())
    //             {
    //                 Ok(value) => Ok(GetHttpResponseVariants::from(value)),
    //                 Err(e) => Err(
    //                     GetHttpResponseVariantsTryFromReqwestResponseVariant::DeserializeResponse {
    //                         reqwest: e,
    //                         status_code,
    //                     },
    //                 ),
    //             }
    //         } else if status_code == http::StatusCode::INTERNAL_SERVER_ERROR {
    //             match futures::executor::block_on(
    //                 response.json::<GetHttpResponseVariantsInternalServerError>(),
    //             ) {
    //                 Ok(value) => Ok(GetHttpResponseVariants::from(value)),
    //                 Err(e) => Err(
    //                     GetHttpResponseVariantsTryFromReqwestResponseVariant::DeserializeResponse {
    //                         reqwest: e,
    //                         status_code,
    //                     },
    //                 ),
    //             }
    //         } else if status_code == http::StatusCode::NOT_FOUND {
    //             match futures::executor::block_on(response.json::<GetHttpResponseVariantsNotFound>()) {
    //                 Ok(value) => Ok(GetHttpResponseVariants::from(value)),
    //                 Err(e) => Err(
    //                     GetHttpResponseVariantsTryFromReqwestResponseVariant::DeserializeResponse {
    //                         reqwest: e,
    //                         status_code,
    //                     },
    //                 ),
    //             }
    //         } else if status_code == http::StatusCode::REQUEST_TIMEOUT {
    //             match futures::executor::block_on(
    //                 response.json::<GetHttpResponseVariantsRequestTimeout>(),
    //             ) {
    //                 Ok(value) => Ok(GetHttpResponseVariants::from(value)),
    //                 Err(e) => Err(
    //                     GetHttpResponseVariantsTryFromReqwestResponseVariant::DeserializeResponse {
    //                         reqwest: e,
    //                         status_code,
    //                     },
    //                 ),
    //             }
    //         } else {
    //             Err(
    //                 GetHttpResponseVariantsTryFromReqwestResponseVariant::UnexpectedStatusCode {
    //                     status_code,
    //                 },
    //             )
    //         }
    //     }
    // }

        };
    //println!("{gen}");
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
    fn to_http_status_code_quote<'a>(&self) -> proc_macro2::TokenStream {
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

// impl Attribute {
//     fn to_str_attribute<'a>(&self) -> &'a str {
//         match self {
//             Attribute::Tvfrr100 => "tvfrr_100",
//             Attribute::Tvfrr101 => "tvfrr_101",
//             Attribute::Tvfrr102 => "tvfrr_102",
//             Attribute::Tvfrr200 => "tvfrr_200",
//             Attribute::Tvfrr201 => "tvfrr_201",
//             Attribute::Tvfrr202 => "tvfrr_202",
//             Attribute::Tvfrr203 => "tvfrr_203",
//             Attribute::Tvfrr204 => "tvfrr_204",
//             Attribute::Tvfrr205 => "tvfrr_205",
//             Attribute::Tvfrr206 => "tvfrr_206",
//             Attribute::Tvfrr207 => "tvfrr_207",
//             Attribute::Tvfrr208 => "tvfrr_208",
//             Attribute::Tvfrr226 => "tvfrr_226",
//             Attribute::Tvfrr300 => "tvfrr_300",
//             Attribute::Tvfrr301 => "tvfrr_301",
//             Attribute::Tvfrr302 => "tvfrr_302",
//             Attribute::Tvfrr303 => "tvfrr_303",
//             Attribute::Tvfrr304 => "tvfrr_304",
//             Attribute::Tvfrr305 => "tvfrr_305",
//             Attribute::Tvfrr307 => "tvfrr_307",
//             Attribute::Tvfrr308 => "tvfrr_308",
//             Attribute::Tvfrr400 => "tvfrr_400",
//             Attribute::Tvfrr401 => "tvfrr_401",
//             Attribute::Tvfrr402 => "tvfrr_402",
//             Attribute::Tvfrr403 => "tvfrr_403",
//             Attribute::Tvfrr404 => "tvfrr_404",
//             Attribute::Tvfrr405 => "tvfrr_405",
//             Attribute::Tvfrr406 => "tvfrr_406",
//             Attribute::Tvfrr407 => "tvfrr_407",
//             Attribute::Tvfrr408 => "tvfrr_408",
//             Attribute::Tvfrr409 => "tvfrr_409",
//             Attribute::Tvfrr410 => "tvfrr_410",
//             Attribute::Tvfrr411 => "tvfrr_411",
//             Attribute::Tvfrr412 => "tvfrr_412",
//             Attribute::Tvfrr413 => "tvfrr_413",
//             Attribute::Tvfrr414 => "tvfrr_414",
//             Attribute::Tvfrr415 => "tvfrr_415",
//             Attribute::Tvfrr416 => "tvfrr_416",
//             Attribute::Tvfrr417 => "tvfrr_417",
//             Attribute::Tvfrr418 => "tvfrr_418",
//             Attribute::Tvfrr421 => "tvfrr_421",
//             Attribute::Tvfrr422 => "tvfrr_422",
//             Attribute::Tvfrr423 => "tvfrr_423",
//             Attribute::Tvfrr424 => "tvfrr_424",
//             Attribute::Tvfrr426 => "tvfrr_426",
//             Attribute::Tvfrr428 => "tvfrr_428",
//             Attribute::Tvfrr429 => "tvfrr_429",
//             Attribute::Tvfrr431 => "tvfrr_431",
//             Attribute::Tvfrr451 => "tvfrr_451",
//             Attribute::Tvfrr500 => "tvfrr_500",
//             Attribute::Tvfrr501 => "tvfrr_501",
//             Attribute::Tvfrr502 => "tvfrr_502",
//             Attribute::Tvfrr503 => "tvfrr_503",
//             Attribute::Tvfrr504 => "tvfrr_504",
//             Attribute::Tvfrr505 => "tvfrr_505",
//             Attribute::Tvfrr506 => "tvfrr_506",
//             Attribute::Tvfrr507 => "tvfrr_507",
//             Attribute::Tvfrr508 => "tvfrr_508",
//             Attribute::Tvfrr510 => "tvfrr_510",
//             Attribute::Tvfrr511 => "tvfrr_511",
//         }
//     }
// }

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
