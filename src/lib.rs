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
    data_enum.variants.into_iter().for_each(|variant| {
        println!("-----------");
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
        println!("{attr:?}");
        // match variant.fields {
        //     syn::Fields::Named(fields_named) => {}
        //     syn::Fields::Unnamed(fields_unnamed) => {}
        //     syn::Fields::Unit => panic!("{macro_name} does not support syn::Fields::Unit"),
        // };
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

#[derive(Debug)]
enum Attribute {
    Tvfrr100,
    Tvfrr101,
    Tvfrr102,
    Tvfrr200,
    Tvfrr201,
    Tvfrr202,
    Tvfrr203,
    Tvfrr204,
    Tvfrr205,
    Tvfrr206,
    Tvfrr207,
    Tvfrr208,
    Tvfrr226,
    Tvfrr300,
    Tvfrr301,
    Tvfrr302,
    Tvfrr303,
    Tvfrr304,
    Tvfrr305,
    Tvfrr307,
    Tvfrr308,
    Tvfrr400,
    Tvfrr401,
    Tvfrr402,
    Tvfrr403,
    Tvfrr404,
    Tvfrr405,
    Tvfrr406,
    Tvfrr407,
    Tvfrr408,
    Tvfrr409,
    Tvfrr410,
    Tvfrr411,
    Tvfrr412,
    Tvfrr413,
    Tvfrr414,
    Tvfrr415,
    Tvfrr416,
    Tvfrr417,
    Tvfrr418,
    Tvfrr421,
    Tvfrr422,
    Tvfrr423,
    Tvfrr424,
    Tvfrr426,
    Tvfrr428,
    Tvfrr429,
    Tvfrr431,
    Tvfrr451,
    Tvfrr500,
    Tvfrr501,
    Tvfrr502,
    Tvfrr503,
    Tvfrr504,
    Tvfrr505,
    Tvfrr506,
    Tvfrr507,
    Tvfrr508,
    Tvfrr510,
    Tvfrr511,
}

impl Attribute {
    fn to_http_status_code_quote<'a>(&self) -> proc_macro2::TokenStream {
        match self {
            Attribute::Tvfrr100 => quote::quote! {http::StatusCode::CONTINUE},
            Attribute::Tvfrr101 => quote::quote! {http::StatusCode::SWITCHING_PROTOCOLS},
            Attribute::Tvfrr102 => quote::quote! {http::StatusCode::PROCESSING},
            Attribute::Tvfrr200 => quote::quote! {http::StatusCode::OK},
            Attribute::Tvfrr201 => quote::quote! {http::StatusCode::CREATED},
            Attribute::Tvfrr202 => quote::quote! {http::StatusCode::ACCEPTED},
            Attribute::Tvfrr203 => {
                quote::quote! {http::StatusCode::NON_AUTHORITATIVE_INFORMATION}
            }
            Attribute::Tvfrr204 => quote::quote! {http::StatusCode::NO_CONTENT},
            Attribute::Tvfrr205 => quote::quote! {http::StatusCode::RESET_CONTENT},
            Attribute::Tvfrr206 => quote::quote! {http::StatusCode::PARTIAL_CONTENT},
            Attribute::Tvfrr207 => quote::quote! {http::StatusCode::MULTI_STATUS},
            Attribute::Tvfrr208 => quote::quote! {http::StatusCode::ALREADY_REPORTED},
            Attribute::Tvfrr226 => quote::quote! {http::StatusCode::IM_USED},
            Attribute::Tvfrr300 => quote::quote! {http::StatusCode::MULTIPLE_CHOICES},
            Attribute::Tvfrr301 => quote::quote! {http::StatusCode::MOVED_PERMANENTLY},
            Attribute::Tvfrr302 => quote::quote! {http::StatusCode::FOUND},
            Attribute::Tvfrr303 => quote::quote! {http::StatusCode::SEE_OTHER},
            Attribute::Tvfrr304 => quote::quote! {http::StatusCode::NOT_MODIFIED},
            Attribute::Tvfrr305 => quote::quote! {http::StatusCode::USE_PROXY},
            Attribute::Tvfrr307 => quote::quote! {http::StatusCode::TEMPORARY_REDIRECT},
            Attribute::Tvfrr308 => quote::quote! {http::StatusCode::PERMANENT_REDIRECT},
            Attribute::Tvfrr400 => quote::quote! {http::StatusCode::BAD_REQUEST},
            Attribute::Tvfrr401 => quote::quote! {http::StatusCode::UNAUTHORIZED},
            Attribute::Tvfrr402 => quote::quote! {http::StatusCode::PAYMENT_REQUIRED},
            Attribute::Tvfrr403 => quote::quote! {http::StatusCode::FORBIDDEN},
            Attribute::Tvfrr404 => quote::quote! {http::StatusCode::NOT_FOUND},
            Attribute::Tvfrr405 => quote::quote! {http::StatusCode::METHOD_NOT_ALLOWED},
            Attribute::Tvfrr406 => quote::quote! {http::StatusCode::NOT_ACCEPTABLE},
            Attribute::Tvfrr407 => {
                quote::quote! {http::StatusCode::PROXY_AUTHENTICATION_REQUIRED}
            }
            Attribute::Tvfrr408 => quote::quote! {http::StatusCode::REQUEST_TIMEOUT},
            Attribute::Tvfrr409 => quote::quote! {http::StatusCode::CONFLICT},
            Attribute::Tvfrr410 => quote::quote! {http::StatusCode::GONE},
            Attribute::Tvfrr411 => quote::quote! {http::StatusCode::LENGTH_REQUIRED},
            Attribute::Tvfrr412 => quote::quote! {http::StatusCode::PRECONDITION_FAILED},
            Attribute::Tvfrr413 => quote::quote! {http::StatusCode::PAYLOAD_TOO_LARGE},
            Attribute::Tvfrr414 => quote::quote! {http::StatusCode::URI_TOO_LONG},
            Attribute::Tvfrr415 => quote::quote! {http::StatusCode::UNSUPPORTED_MEDIA_TYPE},
            Attribute::Tvfrr416 => quote::quote! {http::StatusCode::RANGE_NOT_SATISFIABLE},
            Attribute::Tvfrr417 => quote::quote! {http::StatusCode::EXPECTATION_FAILED},
            Attribute::Tvfrr418 => quote::quote! {http::StatusCode::IM_A_TEAPOT},
            Attribute::Tvfrr421 => quote::quote! {http::StatusCode::MISDIRECTED_REQUEST},
            Attribute::Tvfrr422 => quote::quote! {http::StatusCode::UNPROCESSABLE_ENTITY},
            Attribute::Tvfrr423 => quote::quote! {http::StatusCode::LOCKED},
            Attribute::Tvfrr424 => quote::quote! {http::StatusCode::FAILED_DEPENDENCY},
            Attribute::Tvfrr426 => quote::quote! {http::StatusCode::UPGRADE_REQUIRED},
            Attribute::Tvfrr428 => quote::quote! {http::StatusCode::PRECONDITION_REQUIRED},
            Attribute::Tvfrr429 => quote::quote! {http::StatusCode::TOO_MANY_REQUESTS},
            Attribute::Tvfrr431 => {
                quote::quote! {http::StatusCode::REQUEST_HEADER_FIELDS_TOO_LARGE}
            }
            Attribute::Tvfrr451 => {
                quote::quote! {http::StatusCode::UNAVAILABLE_FOR_LEGAL_REASONS}
            }
            Attribute::Tvfrr500 => quote::quote! {http::StatusCode::INTERNAL_SERVER_ERROR},
            Attribute::Tvfrr501 => quote::quote! {http::StatusCode::NOT_IMPLEMENTED},
            Attribute::Tvfrr502 => quote::quote! {http::StatusCode::BAD_GATEWAY},
            Attribute::Tvfrr503 => quote::quote! {http::StatusCode::SERVICE_UNAVAILABLE},
            Attribute::Tvfrr504 => quote::quote! {http::StatusCode::GATEWAY_TIMEOUT},
            Attribute::Tvfrr505 => {
                quote::quote! {http::StatusCode::HTTP_VERSION_NOT_SUPPORTED}
            }
            Attribute::Tvfrr506 => quote::quote! {http::StatusCode::VARIANT_ALSO_NEGOTIATES},
            Attribute::Tvfrr507 => quote::quote! {http::StatusCode::INSUFFICIENT_STORAGE},
            Attribute::Tvfrr508 => quote::quote! {http::StatusCode::LOOP_DETECTED},
            Attribute::Tvfrr510 => quote::quote! {http::StatusCode::NOT_EXTENDED},
            Attribute::Tvfrr511 => {
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
            Ok(Attribute::Tvfrr100_continue)
        } else if value == "tvfrr_101_switching_protocols" {
            Ok(Attribute::Tvfrr101_switching_protocols)
        } else if value == "tvfrr_102_processing" {
            Ok(Attribute::Tvfrr102_processing)
        } else if value == "tvfrr_200_ok" {
            Ok(Attribute::Tvfrr200_ok)
        } else if value == "tvfrr_201_created" {
            Ok(Attribute::Tvfrr201_created)
        } else if value == "tvfrr_202_accepted" {
            Ok(Attribute::Tvfrr202_accepted)
        } else if value == "tvfrr_203_non_authoritative_information" {
            Ok(Attribute::Tvfrr203_non_authoritative_information)
        } else if value == "tvfrr_204_no_content" {
            Ok(Attribute::Tvfrr204_no_content)
        } else if value == "tvfrr_205_reset_content" {
            Ok(Attribute::Tvfrr205_reset_content)
        } else if value == "tvfrr_206_partial_content" {
            Ok(Attribute::Tvfrr206_partial_content)
        } else if value == "tvfrr_207_multi_status" {
            Ok(Attribute::Tvfrr207_multi_status)
        } else if value == "tvfrr_208_already_reported" {
            Ok(Attribute::Tvfrr208_already_reported)
        } else if value == "tvfrr_226_im_used" {
            Ok(Attribute::Tvfrr226_im_used)
        } else if value == "tvfrr_300_multiple_choices" {
            Ok(Attribute::Tvfrr300_multiple_choices)
        } else if value == "tvfrr_301_moved_permanently" {
            Ok(Attribute::Tvfrr301_moved_permanently)
        } else if value == "tvfrr_302_found" {
            Ok(Attribute::Tvfrr302_found)
        } else if value == "tvfrr_303_see_other" {
            Ok(Attribute::Tvfrr303_see_other)
        } else if value == "tvfrr_304_not_modified" {
            Ok(Attribute::Tvfrr304_not_modified)
        } else if value == "tvfrr_305_use_proxy" {
            Ok(Attribute::Tvfrr305_use_proxy)
        } else if value == "tvfrr_307_temporary_redirect" {
            Ok(Attribute::Tvfrr307_temporary_redirect)
        } else if value == "tvfrr_308_permanent_redirect" {
            Ok(Attribute::Tvfrr308_permanent_redirect)
        } else if value == "tvfrr_400_bad_request" {
            Ok(Attribute::Tvfrr400_bad_request)
        } else if value == "tvfrr_401_unauthorized" {
            Ok(Attribute::Tvfrr401_unauthorized)
        } else if value == "tvfrr_402_payment_required" {
            Ok(Attribute::Tvfrr402_payment_required)
        } else if value == "tvfrr_403_forbidden" {
            Ok(Attribute::Tvfrr403_forbidden)
        } else if value == "tvfrr_404_not_found" {
            Ok(Attribute::Tvfrr404_not_found)
        } else if value == "tvfrr_405_method_not_allowed" {
            Ok(Attribute::Tvfrr405_method_not_allowed)
        } else if value == "tvfrr_406_not_acceptable" {
            Ok(Attribute::Tvfrr406_not_acceptable)
        } else if value == "tvfrr_407_proxy_authentication_required" {
            Ok(Attribute::Tvfrr407_proxy_authentication_required)
        } else if value == "tvfrr_408_request_timeout" {
            Ok(Attribute::Tvfrr408_request_timeout)
        } else if value == "tvfrr_409_conflict" {
            Ok(Attribute::Tvfrr409_conflict)
        } else if value == "tvfrr_410_gone" {
            Ok(Attribute::Tvfrr410_gone)
        } else if value == "tvfrr_411_length_required" {
            Ok(Attribute::Tvfrr411_length_required)
        } else if value == "tvfrr_412_precondition_failed" {
            Ok(Attribute::Tvfrr412_precondition_failed)
        } else if value == "tvfrr_413_payload_too_large" {
            Ok(Attribute::Tvfrr413_payload_too_large)
        } else if value == "tvfrr_414_uri_too_long" {
            Ok(Attribute::Tvfrr414_uri_too_long)
        } else if value == "tvfrr_415_unsupported_media_type" {
            Ok(Attribute::Tvfrr415_unsupported_media_type)
        } else if value == "tvfrr_416_range_not_satisfiable" {
            Ok(Attribute::Tvfrr416_range_not_satisfiable)
        } else if value == "tvfrr_417_expectation_failed" {
            Ok(Attribute::Tvfrr417_expectation_failed)
        } else if value == "tvfrr_418_im_a_teapot" {
            Ok(Attribute::Tvfrr418_im_a_teapot)
        } else if value == "tvfrr_421_misdirected_request" {
            Ok(Attribute::Tvfrr421_misdirected_request)
        } else if value == "tvfrr_422_unprocessable_entity" {
            Ok(Attribute::Tvfrr422_unprocessable_entity)
        } else if value == "tvfrr_423_locked" {
            Ok(Attribute::Tvfrr423_locked)
        } else if value == "tvfrr_424_failed_dependency" {
            Ok(Attribute::Tvfrr424_failed_dependency)
        } else if value == "tvfrr_426_upgrade_required" {
            Ok(Attribute::Tvfrr426_upgrade_required)
        } else if value == "tvfrr_428_precondition_required" {
            Ok(Attribute::Tvfrr428_precondition_required)
        } else if value == "tvfrr_429_too_many_requests" {
            Ok(Attribute::Tvfrr429_too_many_requests)
        } else if value == "tvfrr_431_request_header_fields_too_large" {
            Ok(Attribute::Tvfrr431_request_header_fields_too_large)
        } else if value == "tvfrr_451_unavailable_for_legal_reasons" {
            Ok(Attribute::Tvfrr451_unavailable_for_legal_reasons)
        } else if value == "tvfrr_500_internal_server_error" {
            Ok(Attribute::Tvfrr500_internal_server_error)
        } else if value == "tvfrr_501_not_implemented" {
            Ok(Attribute::Tvfrr501_not_implemented)
        } else if value == "tvfrr_502_bad_gateway" {
            Ok(Attribute::Tvfrr502_bad_gateway)
        } else if value == "tvfrr_503_service_unavailable" {
            Ok(Attribute::Tvfrr503_service_unavailable)
        } else if value == "tvfrr_504_gateway_timeout" {
            Ok(Attribute::Tvfrr504_gateway_timeout)
        } else if value == "tvfrr_505_http_version_not_supported" {
            Ok(Attribute::Tvfrr505_http_version_not_supported)
        } else if value == "tvfrr_506_variant_also_negotiates" {
            Ok(Attribute::Tvfrr506_variant_also_negotiates)
        } else if value == "tvfrr_507_insufficient_storage" {
            Ok(Attribute::Tvfrr507_insufficient_storage)
        } else if value == "tvfrr_508_loop_detected" {
            Ok(Attribute::Tvfrr508_loop_detected)
        } else if value == "tvfrr_510_not_extended" {
            Ok(Attribute::Tvfrr510_not_extended)
        } else if value == "tvfrr_511_network_authentication_required" {
            Ok(Attribute::Tvfrr511_network_authentication_required)
        } else {
            Err(())
        }
    }
}
