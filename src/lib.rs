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
        panic!("{macro_name} syn::Data is not a syn::Data::Enum");
    };
    data_enum.variants.into_iter().for_each(|variant| {
        let mut option_attribute = None;
        variant.attrs.iter().for_each(|attr| {
            if let true = attr.path.segments.len() == 1 {
                let error_message = "todo";
                if let Ok(named_attribute) = NamedAttribute::try_from(&attr.path.segments[0].ident)
                {
                    if let true = option_attribute.is_some() {
                        panic!("{error_message}");
                    } else {
                        option_attribute = Some(named_attribute);
                    }
                }
            }
            println!("tokens{:#?}token", attr);
        });
        println!("{option_attribute:#?}");
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

#[derive(Debug)]
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

// impl NamedAttribute {
//     fn to_str_attribute<'a>(&self) -> &'a str {
//         match self {
//             NamedAttribute::Tvfrr100 => "tvfrr_100",
//             NamedAttribute::Tvfrr101 => "tvfrr_101",
//             NamedAttribute::Tvfrr102 => "tvfrr_102",
//             NamedAttribute::Tvfrr200 => "tvfrr_200",
//             NamedAttribute::Tvfrr201 => "tvfrr_201",
//             NamedAttribute::Tvfrr202 => "tvfrr_202",
//             NamedAttribute::Tvfrr203 => "tvfrr_203",
//             NamedAttribute::Tvfrr204 => "tvfrr_204",
//             NamedAttribute::Tvfrr205 => "tvfrr_205",
//             NamedAttribute::Tvfrr206 => "tvfrr_206",
//             NamedAttribute::Tvfrr207 => "tvfrr_207",
//             NamedAttribute::Tvfrr208 => "tvfrr_208",
//             NamedAttribute::Tvfrr226 => "tvfrr_226",
//             NamedAttribute::Tvfrr300 => "tvfrr_300",
//             NamedAttribute::Tvfrr301 => "tvfrr_301",
//             NamedAttribute::Tvfrr302 => "tvfrr_302",
//             NamedAttribute::Tvfrr303 => "tvfrr_303",
//             NamedAttribute::Tvfrr304 => "tvfrr_304",
//             NamedAttribute::Tvfrr305 => "tvfrr_305",
//             NamedAttribute::Tvfrr307 => "tvfrr_307",
//             NamedAttribute::Tvfrr308 => "tvfrr_308",
//             NamedAttribute::Tvfrr400 => "tvfrr_400",
//             NamedAttribute::Tvfrr401 => "tvfrr_401",
//             NamedAttribute::Tvfrr402 => "tvfrr_402",
//             NamedAttribute::Tvfrr403 => "tvfrr_403",
//             NamedAttribute::Tvfrr404 => "tvfrr_404",
//             NamedAttribute::Tvfrr405 => "tvfrr_405",
//             NamedAttribute::Tvfrr406 => "tvfrr_406",
//             NamedAttribute::Tvfrr407 => "tvfrr_407",
//             NamedAttribute::Tvfrr408 => "tvfrr_408",
//             NamedAttribute::Tvfrr409 => "tvfrr_409",
//             NamedAttribute::Tvfrr410 => "tvfrr_410",
//             NamedAttribute::Tvfrr411 => "tvfrr_411",
//             NamedAttribute::Tvfrr412 => "tvfrr_412",
//             NamedAttribute::Tvfrr413 => "tvfrr_413",
//             NamedAttribute::Tvfrr414 => "tvfrr_414",
//             NamedAttribute::Tvfrr415 => "tvfrr_415",
//             NamedAttribute::Tvfrr416 => "tvfrr_416",
//             NamedAttribute::Tvfrr417 => "tvfrr_417",
//             NamedAttribute::Tvfrr418 => "tvfrr_418",
//             NamedAttribute::Tvfrr421 => "tvfrr_421",
//             NamedAttribute::Tvfrr422 => "tvfrr_422",
//             NamedAttribute::Tvfrr423 => "tvfrr_423",
//             NamedAttribute::Tvfrr424 => "tvfrr_424",
//             NamedAttribute::Tvfrr426 => "tvfrr_426",
//             NamedAttribute::Tvfrr428 => "tvfrr_428",
//             NamedAttribute::Tvfrr429 => "tvfrr_429",
//             NamedAttribute::Tvfrr431 => "tvfrr_431",
//             NamedAttribute::Tvfrr451 => "tvfrr_451",
//             NamedAttribute::Tvfrr500 => "tvfrr_500",
//             NamedAttribute::Tvfrr501 => "tvfrr_501",
//             NamedAttribute::Tvfrr502 => "tvfrr_502",
//             NamedAttribute::Tvfrr503 => "tvfrr_503",
//             NamedAttribute::Tvfrr504 => "tvfrr_504",
//             NamedAttribute::Tvfrr505 => "tvfrr_505",
//             NamedAttribute::Tvfrr506 => "tvfrr_506",
//             NamedAttribute::Tvfrr507 => "tvfrr_507",
//             NamedAttribute::Tvfrr508 => "tvfrr_508",
//             NamedAttribute::Tvfrr510 => "tvfrr_510",
//             NamedAttribute::Tvfrr511 => "tvfrr_511",
//         }
//     }
// }

impl TryFrom<&syn::Ident> for NamedAttribute {
    type Error = ();
    fn try_from(value: &syn::Ident) -> Result<Self, Self::Error> {
        if value == "tvfrr_100" {
            Ok(NamedAttribute::Tvfrr100)
        } else if value == "tvfrr_101" {
            Ok(NamedAttribute::Tvfrr101)
        } else if value == "tvfrr_102" {
            Ok(NamedAttribute::Tvfrr102)
        } else if value == "tvfrr_200" {
            Ok(NamedAttribute::Tvfrr200)
        } else if value == "tvfrr_201" {
            Ok(NamedAttribute::Tvfrr201)
        } else if value == "tvfrr_202" {
            Ok(NamedAttribute::Tvfrr202)
        } else if value == "tvfrr_203" {
            Ok(NamedAttribute::Tvfrr203)
        } else if value == "tvfrr_204" {
            Ok(NamedAttribute::Tvfrr204)
        } else if value == "tvfrr_205" {
            Ok(NamedAttribute::Tvfrr205)
        } else if value == "tvfrr_206" {
            Ok(NamedAttribute::Tvfrr206)
        } else if value == "tvfrr_207" {
            Ok(NamedAttribute::Tvfrr207)
        } else if value == "tvfrr_208" {
            Ok(NamedAttribute::Tvfrr208)
        } else if value == "tvfrr_226" {
            Ok(NamedAttribute::Tvfrr226)
        } else if value == "tvfrr_300" {
            Ok(NamedAttribute::Tvfrr300)
        } else if value == "tvfrr_301" {
            Ok(NamedAttribute::Tvfrr301)
        } else if value == "tvfrr_302" {
            Ok(NamedAttribute::Tvfrr302)
        } else if value == "tvfrr_303" {
            Ok(NamedAttribute::Tvfrr303)
        } else if value == "tvfrr_304" {
            Ok(NamedAttribute::Tvfrr304)
        } else if value == "tvfrr_305" {
            Ok(NamedAttribute::Tvfrr305)
        } else if value == "tvfrr_307" {
            Ok(NamedAttribute::Tvfrr307)
        } else if value == "tvfrr_308" {
            Ok(NamedAttribute::Tvfrr308)
        } else if value == "tvfrr_400" {
            Ok(NamedAttribute::Tvfrr400)
        } else if value == "tvfrr_401" {
            Ok(NamedAttribute::Tvfrr401)
        } else if value == "tvfrr_402" {
            Ok(NamedAttribute::Tvfrr402)
        } else if value == "tvfrr_403" {
            Ok(NamedAttribute::Tvfrr403)
        } else if value == "tvfrr_404" {
            Ok(NamedAttribute::Tvfrr404)
        } else if value == "tvfrr_405" {
            Ok(NamedAttribute::Tvfrr405)
        } else if value == "tvfrr_406" {
            Ok(NamedAttribute::Tvfrr406)
        } else if value == "tvfrr_407" {
            Ok(NamedAttribute::Tvfrr407)
        } else if value == "tvfrr_408" {
            Ok(NamedAttribute::Tvfrr408)
        } else if value == "tvfrr_409" {
            Ok(NamedAttribute::Tvfrr409)
        } else if value == "tvfrr_410" {
            Ok(NamedAttribute::Tvfrr410)
        } else if value == "tvfrr_411" {
            Ok(NamedAttribute::Tvfrr411)
        } else if value == "tvfrr_412" {
            Ok(NamedAttribute::Tvfrr412)
        } else if value == "tvfrr_413" {
            Ok(NamedAttribute::Tvfrr413)
        } else if value == "tvfrr_414" {
            Ok(NamedAttribute::Tvfrr414)
        } else if value == "tvfrr_415" {
            Ok(NamedAttribute::Tvfrr415)
        } else if value == "tvfrr_416" {
            Ok(NamedAttribute::Tvfrr416)
        } else if value == "tvfrr_417" {
            Ok(NamedAttribute::Tvfrr417)
        } else if value == "tvfrr_418" {
            Ok(NamedAttribute::Tvfrr418)
        } else if value == "tvfrr_421" {
            Ok(NamedAttribute::Tvfrr421)
        } else if value == "tvfrr_422" {
            Ok(NamedAttribute::Tvfrr422)
        } else if value == "tvfrr_423" {
            Ok(NamedAttribute::Tvfrr423)
        } else if value == "tvfrr_424" {
            Ok(NamedAttribute::Tvfrr424)
        } else if value == "tvfrr_426" {
            Ok(NamedAttribute::Tvfrr426)
        } else if value == "tvfrr_428" {
            Ok(NamedAttribute::Tvfrr428)
        } else if value == "tvfrr_429" {
            Ok(NamedAttribute::Tvfrr429)
        } else if value == "tvfrr_431" {
            Ok(NamedAttribute::Tvfrr431)
        } else if value == "tvfrr_451" {
            Ok(NamedAttribute::Tvfrr451)
        } else if value == "tvfrr_500" {
            Ok(NamedAttribute::Tvfrr500)
        } else if value == "tvfrr_501" {
            Ok(NamedAttribute::Tvfrr501)
        } else if value == "tvfrr_502" {
            Ok(NamedAttribute::Tvfrr502)
        } else if value == "tvfrr_503" {
            Ok(NamedAttribute::Tvfrr503)
        } else if value == "tvfrr_504" {
            Ok(NamedAttribute::Tvfrr504)
        } else if value == "tvfrr_505" {
            Ok(NamedAttribute::Tvfrr505)
        } else if value == "tvfrr_506" {
            Ok(NamedAttribute::Tvfrr506)
        } else if value == "tvfrr_507" {
            Ok(NamedAttribute::Tvfrr507)
        } else if value == "tvfrr_508" {
            Ok(NamedAttribute::Tvfrr508)
        } else if value == "tvfrr_510" {
            Ok(NamedAttribute::Tvfrr510)
        } else if value == "tvfrr_511" {
            Ok(NamedAttribute::Tvfrr511)
        } else {
            Err(())
        }
    }
}
