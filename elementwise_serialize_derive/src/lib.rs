use proc_macro::TokenStream;
use syn::{punctuated::Punctuated, token::Comma, Data, DataStruct, DeriveInput, Field, Fields};
#[macro_use]
extern crate quote;

#[proc_macro_derive(ElementwiseSerialize)]
pub fn elementwise_serialize_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree.
    let ast = syn::parse(input).expect("Rust code should be parseable");

    // Build the trait implementation.
    impl_elementwise_serialize(&ast)
}

fn impl_elementwise_serialize(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let field_name_iter = fields(ast).iter().map(|field| &field.ident);

    // Note: field types are accessible but are not useful for determining if an Option type:
    // let field_type_iter = fields.iter().map(|field| &field.ty);

    // Generate the code to implement the ElementwiseSerialize trait.
    let gen = quote! {
        impl ElementwiseSerialize for #name {
            fn elementwise_serialize(&self, path: &std::path::Path) -> Result<(), std::io::Error> {
                #(
                    // Construct the filename from the struct field.
                    let filename = format!("{}.json", stringify!(#field_name_iter));
                    let full_path = path.join(filename);

                    // Skip any field whose value is None (which serde serializes to `null`).
                    let value = &self.#field_name_iter;
                    if serde_json::to_string(&value)? != "null" {

                        // Check for an existing file. Do not overwrite.
                        if !full_path.exists() {
                            let mut filename = stringify!(#field_name_iter).to_string();
                            let writer = std::fs::File::create_new(&full_path)?;
                            serde_json::to_writer_pretty(&writer, &self.#field_name_iter)?;

                            // Set file permissions to read only.
                            let mut permissions = std::fs::metadata(&full_path)?.permissions();
                            permissions.set_readonly(true);
                            std::fs::set_permissions(&full_path, permissions)?;
                        }
                    }
                )*
                Ok(())
            }
        }
    };
    gen.into()
}

#[proc_macro_derive(ElementwiseDeserialize)]
pub fn elementwise_deserialize_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree.
    let ast = syn::parse(input).expect("Rust code should be parseable");

    // Build the trait implementation.
    impl_elementwise_deserialize(&ast)
}

fn impl_elementwise_deserialize(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let fields = fields(ast);
    let field_name_iter = fields.iter().map(|field| &field.ident);
    let field_name_iter_clone = field_name_iter.clone();
    let field_type_iter = fields.iter().map(|field| &field.ty);

    let gen = quote! {
        impl<'a> ElementwiseDeserialize<'a> for #name {
            fn elementwise_deserialize(path: &std::path::Path) -> Result<Self, std::io::Error> {
                #(
                    // Construct the filename from the struct field.
                    let filename = format!("{}.json", stringify!(#field_name_iter));
                    let full_path = path.join(&filename);

                    // Missing files should be interpreted as None values in the struct, so first
                    // deserialize into a String to accommodate both Option and non-Option types.
                    let field_value_str: String =  match full_path.exists() {
                        true => {
                            // If the file is available, deserialize with serde and then serialize to string.
                            if let Ok(file) = std::fs::File::open(&full_path) {
                                let reader = std::io::BufReader::new(file);
                                let #field_name_iter: #field_type_iter = serde_json::from_reader(reader)?;
                                serde_json::to_string(&#field_name_iter)?
                            } else {
                                let msg = format!("File found but failed to open: {:?}", full_path);
                                return Err(std::io::Error::new(std::io::ErrorKind::Other, msg))
                            }
                        },
                        false => {
                            // File not file implies None value, for which the serde serialization is `null`.
                            "null".to_string()
                        }
                    };
                    // Deseserialize the field value string with serde to get the correct type.
                    let #field_name_iter: #field_type_iter = serde_json::from_str(&field_value_str)?;
                )*
                // Construct the struct from the named fields.
                Ok(#name {
                    #(
                        #field_name_iter_clone,
                    )*
                })
            }
        }
    };
    gen.into()
}

fn fields(ast: &DeriveInput) -> &Punctuated<Field, Comma> {
    // Access the struct fields.
    // See https://github.com/dtolnay/syn/issues/516#issuecomment-431637005
    match &ast.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => &fields.named,
        _ => panic!("Expected a struct with named fields"),
    }
}