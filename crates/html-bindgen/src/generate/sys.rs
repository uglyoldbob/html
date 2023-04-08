use super::{CodeFile, Module};
use std::fmt::Write;
use std::{collections::HashMap, iter};

use crate::parse::{Attribute, AttributeType, ParsedElement};
use crate::{utils, Result};
use indoc::{formatdoc, writedoc};

const TRAIT: &str = "
/// Render an element to a writer.
pub trait RenderElement {
    /// Write the opening tag to a writer.
    fn write_opening_tag<W: std::fmt::Write >(&self, writer: &mut W) -> std::fmt::Result;

    /// Write the closing tag to a writer, if one is available.
    fn write_closing_tag<W: std::fmt::Write >(&self, writer: &mut W) -> std::fmt::Result;
}";

pub fn generate(
    parsed: impl Iterator<Item = Result<ParsedElement>>,
    global_attributes: &[Attribute],
    modules: &[Module],
) -> Result<Vec<CodeFile>> {
    let mut output = vec![];
    let mut generated: HashMap<String, Vec<String>> = HashMap::new();

    // generate individual `{element}.rs` files
    for el in parsed {
        let el = el?;
        let entry = generated.entry(el.element_kind.clone());
        entry.or_default().push(el.tag_name.clone());
        let cf = generate_element(el)?;
        output.push(cf);
    }

    // generate `mod.rs` files
    let mut dirs = vec![];
    for (dir, filenames) in generated {
        dirs.push(dir.clone());
        let code = filenames
            .into_iter()
            .map(|name| format!("mod {name};\npub use {name}::*;"))
            .collect::<String>();

        let module = modules.iter().find(|el| &el.name == &dir).unwrap();
        let description = &module.description;
        let code = format!(
            "//! {description}
            {code}"
        );

        output.push(CodeFile {
            filename: "mod.rs".to_owned(),
            code: utils::fmt(dbg!(&code)).expect("could not parse code"),
            dir,
        })
    }
    dirs.sort();

    // generate `lib.rs` file
    let code = dirs
        .into_iter()
        .map(|d| format!("pub mod {d};\n"))
        .chain(iter::once(TRAIT.to_owned()))
        .chain(iter::once({
            let fields = generate_fields(global_attributes);

            let mut display_attrs = String::new();
            for attr in global_attributes {
                display_attrs.push_str(&generate_attribute_display(&attr));
            }
            formatdoc!(
                r#"

                    /// The "global attributes" struct
                    #[derive(Debug, Clone, PartialEq, PartialOrd, Hash, Default)]
                    pub struct GlobalAttributes {{
                        {fields}
                    }}

                    impl std::fmt::Display for GlobalAttributes {{
                        fn fmt(&self, writer: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {{
                            {display_attrs}
                            Ok(())
                        }}
                    }}
                    "#
            )
        }))
        .collect::<String>();
    output.push(CodeFile {
        filename: "lib.rs".to_owned(),
        code: utils::fmt(&code)?,
        dir: String::new(),
    });

    Ok(output)
}

/// Generate a single element.
fn generate_element(el: ParsedElement) -> Result<CodeFile> {
    let dir = el.element_kind.clone();
    let ParsedElement {
        tag_name,
        struct_name,
        has_closing_tag,
        attributes,
        mdn_link,
        has_global_attributes,
        ..
    } = el;

    let filename = format!("{}.rs", tag_name);
    let fields = generate_fields(&attributes);
    let opening_tag_content = generate_opening_tag(&attributes, &tag_name);
    let closing_tag_content = generate_closing_tag(&tag_name, has_closing_tag);

    let global_field = match has_global_attributes {
        true => format!("global_attributes: crate::GlobalAttributes,"),
        false => String::new(),
    };

    let mut code = formatdoc!(
        r#"/// The HTML `<{tag_name}>` element
        ///
        /// [MDN Documentation]({mdn_link})
        #[doc(alias = "{tag_name}")]
        #[non_exhaustive]
        #[derive(Debug, Clone, PartialEq, PartialOrd, Hash, Default)]
        pub struct {struct_name} {{
            {global_field}
            {fields}
        }}

        impl crate::RenderElement for {struct_name} {{
            fn write_opening_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {{
                {opening_tag_content}
                Ok(())
            }}

            #[allow(unused_variables)]
            fn write_closing_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {{
                {closing_tag_content}
                Ok(())
            }}
        }}
    "#
    );

    if has_global_attributes {
        code.push_str(&formatdoc!(
            r#"
            impl std::ops::Deref for {struct_name} {{
                type Target = crate::GlobalAttributes;

                fn deref(&self) -> &Self::Target {{
                    &self.global_attributes
                }}
            }}

            impl std::ops::DerefMut for {struct_name} {{
                fn deref_mut(&mut self) -> &mut Self::Target {{
                    &mut self.global_attributes
                }}
            }}
        "#
        ));
    }

    Ok(CodeFile {
        filename,
        code: utils::fmt(&code)?,
        dir,
    })
}

fn generate_fields(attributes: &[Attribute]) -> String {
    let mut output = String::new();
    for attr in attributes {
        let description = &attr.description;
        let field_name = &attr.field_name;
        let ty = &attr.ty;
        output.push_str(&formatdoc!(
            "/// {description}
             pub {field_name}: std::option::Option<{ty}>,
            "
        ));
    }
    output
}

fn generate_opening_tag(attributes: &[Attribute], tag_name: &str) -> String {
    let mut output = formatdoc!(
        r#"
        write!(writer, "<{tag_name}")?;
    "#
    );
    for attr in attributes {
        output.push_str(&generate_attribute_display(&attr));
    }
    writedoc!(&mut output, r#"write!(writer, ">")?;"#).unwrap();
    output
}

fn generate_closing_tag(tag_name: &str, has_closing_tag: bool) -> String {
    if has_closing_tag {
        formatdoc!(
            r#"write!(writer, "</{tag_name}>")?;
        "#
        )
    } else {
        String::new()
    }
}

fn generate_attribute_display(attr: &Attribute) -> String {
    let Attribute {
        name,
        field_name,
        ty,
        ..
    } = &attr;
    match ty {
        AttributeType::Bool => format!(
            r##"if let Some(field) = self.{field_name}.as_ref() {{
                    if *field {{
                        write!(writer, r#""{name}"#)?;
                    }}
            }}"##
        ),
        AttributeType::String | AttributeType::Integer | AttributeType::Float => format!(
            r##"if let Some(field) = self.{field_name}.as_ref() {{
                write!(writer, r#""{name}="{{field}}""#)?;
            }}"##
        ),
        AttributeType::Identifier(_) => todo!(),
        AttributeType::Enumerable(_) => todo!(),
    }
}
