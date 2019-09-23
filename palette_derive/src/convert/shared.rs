use std::fmt;

use proc_macro2::{Span, TokenStream};
use syn::{parse_quote, GenericParam, Generics, Ident, Path, Turbofish, Type, TypePath};

use meta::KeyValuePair;
use util;

pub fn find_in_generics(
    component: Option<&Type>,
    white_point: Option<&Type>,
    generics: &Generics,
) -> (bool, bool) {
    let mut generic_component = false;
    let mut generic_white_point = false;

    for param in &generics.params {
        if let GenericParam::Type(ref param) = *param {
            if let Some(&Type::Path(TypePath {
                qself: None,
                path:
                    Path {
                        segments: ref component,
                        leading_colon,
                    },
            })) = component
            {
                let first = component.first();
                let is_ident_path = leading_colon.is_none()
                    && component.len() == 1
                    && first.unwrap().arguments.is_empty()
                    && first.unwrap().ident == param.ident;

                if is_ident_path {
                    generic_component = true;
                }
            }

            if let Some(&Type::Path(TypePath {
                qself: None,
                path:
                    Path {
                        segments: ref white_point,
                        leading_colon,
                    },
            })) = white_point
            {
                let first = white_point.first();
                let is_ident_path = leading_colon.is_none()
                    && white_point.len() == 1
                    && first.unwrap().arguments.is_empty()
                    && first.unwrap().ident == param.ident;

                if is_ident_path {
                    generic_white_point = true;
                }
            }
        }
    }

    (generic_component, generic_white_point)
}

pub fn white_point_type(white_point: Option<Type>, internal: bool) -> Type {
    white_point.unwrap_or_else(|| util::path_type(&["white_point", "D65"], internal))
}

pub fn component_type(component: Option<Type>) -> Type {
    component.unwrap_or_else(|| parse_quote!(f32))
}

pub fn rgb_space_type(rgb_space: Option<Type>, white_point: &Type, internal: bool) -> Type {
    rgb_space.unwrap_or_else(|| {
        let srgb_path = util::path_type(&["encoding", "srgb", "Srgb"], internal);
        parse_quote!((#srgb_path, #white_point))
    })
}

pub fn add_component_where_clause(component: &Type, generics: &mut Generics, internal: bool) {
    let component_trait_path = util::path(&["Component"], internal);

    generics
        .make_where_clause()
        .predicates
        .push(parse_quote!(#component: #component_trait_path + _FloatTrait));
}

pub fn add_white_point_where_clause(white_point: &Type, generics: &mut Generics, internal: bool) {
    let white_point_trait_path = util::path(&["white_point", "WhitePoint"], internal);

    generics
        .make_where_clause()
        .predicates
        .push(parse_quote!(#white_point: #white_point_trait_path));
}

pub fn generate_methods(
    ident: &Ident,
    convert_direction: ConvertDirection,
    implementations: &[KeyValuePair],
    component: &Type,
    white_point: &Type,
    rgb_space: &Type,
    turbofish_generics: &Turbofish,
    internal: bool,
) -> Vec<TokenStream> {
    let mut xyz_convert = Some(XyzConvert::Luma);
    let mut methods = vec![];

    for color in implementations {
        if color.key == "Xyz" {
            xyz_convert = None;
        }

        xyz_convert = xyz_convert.map(|current| {
            current.get_best(match &*color.key.to_string() {
                "Rgb" | "Hsl" | "Hsv" | "Hwb" => XyzConvert::Rgb,
                "Lab" => XyzConvert::Lab,
                "Lch" => XyzConvert::Lch,
                "Yxy" => XyzConvert::Yxy,
                "Luma" => XyzConvert::Luma,
                color => panic!("unexpected color type: {}", color),
            })
        });

        let color_name = color.key.to_string();

        let method_name = Ident::new(
            &format!("{}_{}", convert_direction, color_name.to_lowercase()),
            Span::call_site(),
        );
        let color_path = util::color_path(&*color_name, internal);
        let convert_function = color
            .value
            .clone()
            .unwrap_or_else(|| Ident::new(convert_direction.as_ref(), Span::call_site()));

        let method = match &*color_name {
            "Rgb" | "Hsl" | "Hsv" | "Hwb" => {
                let rgb_space_path = util::path(&["rgb", "RgbSpace"], internal);
                quote!(#method_name<_S: #rgb_space_path<WhitePoint = #white_point>>)
            }
            _ => quote!(#method_name),
        };

        let color_ty = match &*color_name {
            "Rgb" => {
                let linear_path = util::path(&["encoding", "Linear"], internal);

                quote!(#color_path<#linear_path<_S>, #component>)
            }
            "Luma" => {
                let linear_path = util::path(&["encoding", "Linear"], internal);

                quote!(#color_path<#linear_path<#white_point>, #component>)
            }
            "Hsl" | "Hsv" | "Hwb" => quote!(#color_path<_S, #component>),
            _ => quote!(#color_path<#white_point, #component>),
        };

        methods.push(match convert_direction {
            ConvertDirection::From => quote! {
                fn #method (color: #color_ty) -> Self {
                    #ident #turbofish_generics::#convert_function(color)
                }
            },
            ConvertDirection::Into => quote! {
                fn #method (self) -> #color_ty {
                    self.#convert_function()
                }
            },
        });
    }

    if let Some(xyz_convert) = xyz_convert {
        let color_path = util::path(&["Xyz"], internal);
        let method_name = Ident::new(&format!("{}_xyz", convert_direction), Span::call_site());
        let into_temporary_name = Ident::new(&format!("into_{}", xyz_convert), Span::call_site());
        let into_color_trait_path = util::path(&["IntoColor"], internal);
        let convert_function = Ident::new(
            &format!("{}_{}", convert_direction, xyz_convert),
            Span::call_site(),
        );

        let method = match convert_direction {
            ConvertDirection::From if xyz_convert == XyzConvert::Rgb => quote! {
                fn #method_name(color: #color_path<#white_point, #component>) -> Self {
                    use #into_color_trait_path;
                    #ident #turbofish_generics::#convert_function(color.#into_temporary_name::<#rgb_space>())
                }
            },
            ConvertDirection::From => quote! {
                fn #method_name(color: #color_path<#white_point, #component>) -> Self {
                    use #into_color_trait_path;
                    #ident #turbofish_generics::#convert_function(color.#into_temporary_name())
                }
            },
            ConvertDirection::Into if xyz_convert == XyzConvert::Rgb => quote! {
                fn #method_name(self) -> #color_path<#white_point, #component> {
                    self.#convert_function::<#rgb_space>().into_xyz()
                }
            },
            ConvertDirection::Into => quote! {
                fn #method_name(self) -> #color_path<#white_point, #component> {
                    self.#convert_function().into_xyz()
                }
            },
        };

        methods.push(method);
    }

    methods
}

pub fn get_convert_color_type(
    color: &str,
    white_point: &Type,
    component: &Type,
    rgb_space: Option<&Type>,
    generics: &mut Generics,
    internal: bool,
) -> Type {
    let color_path = util::color_path(color, internal);

    match color {
        "Rgb" => {
            let rgb_standard_path = util::path(&["rgb", "RgbStandard"], internal);
            let rgb_space_path = util::path(&["rgb", "RgbSpace"], internal);
            generics.params.push(GenericParam::Type(
                Ident::new("_S", Span::call_site()).into(),
            ));

            let where_clause = generics.make_where_clause();
            if let Some(ref rgb_space) = rgb_space {
                where_clause
                    .predicates
                    .push(parse_quote!(_S: #rgb_standard_path<Space = #rgb_space>));
            } else {
                where_clause
                    .predicates
                    .push(parse_quote!(_S: #rgb_standard_path));
                where_clause
                    .predicates
                    .push(parse_quote!(_S::Space: #rgb_space_path<WhitePoint = #white_point>));
            }

            parse_quote!(#color_path<_S, #component>)
        }
        "Luma" => {
            let luma_standard_path = util::path(&["luma", "LumaStandard"], internal);
            generics.params.push(GenericParam::Type(
                Ident::new("_S", Span::call_site()).into(),
            ));

            generics
                .make_where_clause()
                .predicates
                .push(parse_quote!(_S: #luma_standard_path<WhitePoint = #white_point>));
            parse_quote!(#color_path<_S, #component>)
        }
        "Hsl" | "Hsv" | "Hwb" => {
            let rgb_space_path = util::path(&["rgb", "RgbSpace"], internal);

            if let Some(ref rgb_space) = rgb_space {
                parse_quote!(#color_path<#rgb_space, #component>)
            } else {
                generics.params.push(GenericParam::Type(
                    Ident::new("_S", Span::call_site()).into(),
                ));

                generics
                    .make_where_clause()
                    .predicates
                    .push(parse_quote!(_S: #rgb_space_path<WhitePoint = #white_point>));

                parse_quote!(#color_path<_S, #component>)
            }
        }
        _ => parse_quote!(#color_path<#white_point, #component>),
    }
}

#[derive(Clone, Copy)]
pub enum ConvertDirection {
    From,
    Into,
}

impl fmt::Display for ConvertDirection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.as_ref().fmt(f)
    }
}

impl AsRef<str> for ConvertDirection {
    fn as_ref(&self) -> &str {
        match *self {
            ConvertDirection::From => "from",
            ConvertDirection::Into => "into",
        }
    }
}

#[derive(PartialEq, Clone, Copy)]
enum XyzConvert {
    Luma,
    Hwb,
    Hsl,
    Hsv,
    Rgb,
    Lab,
    Lch,
    Yxy,
}

impl XyzConvert {
    fn get_best(&self, other: XyzConvert) -> XyzConvert {
        match (*self, other) {
            (XyzConvert::Yxy, _) | (_, XyzConvert::Yxy) => XyzConvert::Yxy,
            (XyzConvert::Lab, _) | (_, XyzConvert::Lab) => XyzConvert::Lab,
            (XyzConvert::Lch, _) | (_, XyzConvert::Lch) => XyzConvert::Lch,
            (XyzConvert::Rgb, _) | (_, XyzConvert::Rgb) => XyzConvert::Rgb,
            (XyzConvert::Hsl, _) | (_, XyzConvert::Hsl) => XyzConvert::Hsl,
            (XyzConvert::Hsv, _) | (_, XyzConvert::Hsv) => XyzConvert::Hsv,
            (XyzConvert::Hwb, _) | (_, XyzConvert::Hwb) => XyzConvert::Hwb,
            (XyzConvert::Luma, XyzConvert::Luma) => XyzConvert::Luma,
        }
    }
}

impl AsRef<str> for XyzConvert {
    fn as_ref(&self) -> &str {
        match *self {
            XyzConvert::Luma => "luma",
            XyzConvert::Hwb => "hwb",
            XyzConvert::Hsl => "hsl",
            XyzConvert::Hsv => "hsv",
            XyzConvert::Rgb => "rgb",
            XyzConvert::Lab => "lab",
            XyzConvert::Lch => "lch",
            XyzConvert::Yxy => "yxy",
        }
    }
}

impl fmt::Display for XyzConvert {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.as_ref().fmt(f)
    }
}
