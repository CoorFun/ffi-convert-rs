pub fn parse_enum_cases(data: &syn::Data) -> Vec<Variant> {
    match &data {
        syn::Data::Enum(data) => data
            .variants
            .iter()
            .map(parse_variant)
            .collect::<Vec<Variant>>(),
        _ => panic!("CReprOfEnum / AsRustEnum can only be derived for enums"),
    }
}

#[derive(Clone)]
pub struct Pointee {
    pub ty: syn::TypePath,
    pub is_string: bool
}

#[derive(Clone)]
pub struct Variant<'a> {
    pub name: &'a syn::Ident,
    pub case_name: Option<syn::Ident>,
    pub pointee: Option<Pointee>,
    pub is_default: bool,
}

pub fn parse_variant(data: &syn::Variant) -> Variant {
    let name = &data.ident;
    let case_name = data
        .attrs
        .iter()
        .find(|attr| attr.path.get_ident().map(|it| it.to_string()) == Some("case".into()))
        .map(|attr| {
            attr.parse_args()
                .expect("Could not parse attributes of case")
        });

    let pointee_ty: Option<syn::TypePath> = data
        .attrs
        .iter()
        .find(|attr| attr.path.get_ident().map(|it| it.to_string()) == Some("pointee".into()))
        .map(|attr| {
            attr.parse_args()
                .expect("Could not parse attributes of pointee")
        });

    let pointee = if let Some(v) = pointee_ty {
        let is_string = if let Some(segment) = v.path.segments.last() {
            &segment.ident.to_string() == "c_char"
        } else {
            false
        };

        Some(Pointee {
            ty: v,
            is_string
        })
    } else {
        None
    };

    let is_default = data
        .attrs
        .iter()
        .any(|attr| attr.path.get_ident().map(|it| it.to_string()) == Some("default".into()));

    Variant { name, case_name, pointee, is_default }
}
