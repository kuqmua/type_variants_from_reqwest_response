#[proc_macro_derive(
    TypeVariantsFromReqwestResponse,
    attributes(
        tvfrr_100, tvfrr_101, tvfrr_102, tvfrr_200, tvfrr_201, tvfrr_202, tvfrr_203, tvfrr_204,
        tvfrr_205, tvfrr_206, tvfrr_207, tvfrr_208, tvfrr_226, tvfrr_300, tvfrr_301, tvfrr_302,
        tvfrr_303, tvfrr_304, tvfrr_305, tvfrr_307, tvfrr_308, tvfrr_400, tvfrr_401, tvfrr_402,
        tvfrr_403, tvfrr_404, tvfrr_405, tvfrr_406, tvfrr_407, tvfrr_408, tvfrr_409, tvfrr_410,
        tvfrr_411, tvfrr_412, tvfrr_413, tvfrr_414, tvfrr_415, tvfrr_416, tvfrr_417, tvfrr_418,
        tvfrr_421, tvfrr_422, tvfrr_423, tvfrr_424, tvfrr_426, tvfrr_428, tvfrr_429, tvfrr_431,
        tvfrr_451, tvfrr_500, tvfrr_501, tvfrr_502, tvfrr_503, tvfrr_504, tvfrr_505, tvfrr_506,
        tvfrr_507, tvfrr_508, tvfrr_510, tvfrr_511,
    )
)]
pub fn type_variants_from_reqwest_response(
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    proc_macro_helpers::panic_location::panic_location(); //panic_location function from https://github.com/kuqmua/proc_macro_helpers
    let macro_name = "TypeVariantsFromReqwestResponse";
    let tvfrr_100 = "tvfrr_100";
    let tvfrr_101 = "tvfrr_101";
    let tvfrr_102 = "tvfrr_102";
    let tvfrr_200 = "tvfrr_200";
    let tvfrr_201 = "tvfrr_201";
    let tvfrr_202 = "tvfrr_202";
    let tvfrr_203 = "tvfrr_203";
    let tvfrr_204 = "tvfrr_204";
    let tvfrr_205 = "tvfrr_205";
    let tvfrr_206 = "tvfrr_206";
    let tvfrr_207 = "tvfrr_207";
    let tvfrr_208 = "tvfrr_208";
    let tvfrr_226 = "tvfrr_226";
    let tvfrr_300 = "tvfrr_300";
    let tvfrr_301 = "tvfrr_301";
    let tvfrr_302 = "tvfrr_302";
    let tvfrr_303 = "tvfrr_303";
    let tvfrr_304 = "tvfrr_304";
    let tvfrr_305 = "tvfrr_305";
    let tvfrr_307 = "tvfrr_307";
    let tvfrr_308 = "tvfrr_308";
    let tvfrr_400 = "tvfrr_400";
    let tvfrr_401 = "tvfrr_401";
    let tvfrr_402 = "tvfrr_402";
    let tvfrr_403 = "tvfrr_403";
    let tvfrr_404 = "tvfrr_404";
    let tvfrr_405 = "tvfrr_405";
    let tvfrr_406 = "tvfrr_406";
    let tvfrr_407 = "tvfrr_407";
    let tvfrr_408 = "tvfrr_408";
    let tvfrr_409 = "tvfrr_409";
    let tvfrr_410 = "tvfrr_410";
    let tvfrr_411 = "tvfrr_411";
    let tvfrr_412 = "tvfrr_412";
    let tvfrr_413 = "tvfrr_413";
    let tvfrr_414 = "tvfrr_414";
    let tvfrr_415 = "tvfrr_415";
    let tvfrr_416 = "tvfrr_416";
    let tvfrr_417 = "tvfrr_417";
    let tvfrr_418 = "tvfrr_418";
    let tvfrr_421 = "tvfrr_421";
    let tvfrr_422 = "tvfrr_422";
    let tvfrr_423 = "tvfrr_423";
    let tvfrr_424 = "tvfrr_424";
    let tvfrr_426 = "tvfrr_426";
    let tvfrr_428 = "tvfrr_428";
    let tvfrr_429 = "tvfrr_429";
    let tvfrr_431 = "tvfrr_431";
    let tvfrr_451 = "tvfrr_451";
    let tvfrr_500 = "tvfrr_500";
    let tvfrr_501 = "tvfrr_501";
    let tvfrr_502 = "tvfrr_502";
    let tvfrr_503 = "tvfrr_503";
    let tvfrr_504 = "tvfrr_504";
    let tvfrr_505 = "tvfrr_505";
    let tvfrr_506 = "tvfrr_506";
    let tvfrr_507 = "tvfrr_507";
    let tvfrr_508 = "tvfrr_508";
    let tvfrr_510 = "tvfrr_510";
    let tvfrr_511 = "tvfrr_511";

    let ast: syn::DeriveInput = syn::parse(input).unwrap_or_else(|_| {
        panic!("{macro_name} let ast: syn::DeriveInput = syn::parse(input) failed")
    });
    let ident = &ast.ident;
    let data_enum = if let syn::Data::Enum(data_enum) = ast.data {
        data_enum
    } else {
        panic!("{macro_name} syn::Data is not a syn::Data::Enum");
    };
    data_enum.variants.into_iter().for_each(|variant| {
        // println!("{}{:#?}", variant.ident, variant.attrs);
        println!("---------");
        println!("{}", variant.attrs.len());
        let mut option_attribute = None;
        variant.attrs.iter().for_each(|attr| {
            if let true = attr.path.segments.len() == 1 {
                let error_message = "todo";
                if let true = attr.path.segments[0].ident == tvfrr_100 {
                    if let true = option_attribute.is_some() {
                        panic!("{error_message}");
                    } else {
                        option_attribute = Some(NamedAttribute::EoDisplay);
                    }
                } else if let true = attr.path.segments[0].ident == tvfrr_101 {
                    if let true = option_attribute.is_some() {
                        panic!("{error_message}");
                    } else {
                        option_attribute = Some(NamedAttribute::EoDisplayWithSerializeDeserialize);
                    }
                }
            }
            println!("tokens{:#?}token", attr);
        });
        println!("+++++++++");
        match variant.fields {
            syn::Fields::Named(fields_named) => {}
            syn::Fields::Unnamed(fields_unnamed) => {}
            syn::Fields::Unit => panic!("{macro_name} does not support syn::Fields::Unit"),
        }
    });
    let gen = quote::quote! {};
    //println!("{gen}");
    gen.into()
}

enum NamedAttribute {
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
