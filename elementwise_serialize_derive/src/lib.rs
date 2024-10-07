use proc_macro::TokenStream;
use syn::{DataStruct, Fields, Data};
#[macro_use]
extern crate quote;

#[proc_macro_derive(ElementwiseSerialize)]
pub fn elementwise_serialize_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree.
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation.
    impl_elementwise_serialize(&ast)
}

fn impl_elementwise_serialize(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    // Access the struct fields.
    // See https://github.com/dtolnay/syn/issues/516#issuecomment-431637005
    let fields = match &ast.data {
        Data::Struct(DataStruct { fields: Fields::Named(fields), .. }) => &fields.named,
        _ => panic!("Expected a struct with named fields"),
    };
    let field_name_iter = fields.iter().map(|field| &field.ident);
    
    // Note: field types are accessible but are not useful for determining if an Option type:
    // let field_type_iter = fields.iter().map(|field| &field.ty);

    // Generate the code to implement the ElementwiseSerialize trait.
    let gen = quote! {
        impl ElementwiseSerialize for #name {
            fn elementwise_serialize(&self, path: &Path) -> Result<(), Error> {
                #(
                    // Construct the filename from the struct field.
                    let mut filename = stringify!(#field_name_iter).to_string();
                    filename.push_str(".json");
                    let mut full_path = path.clone();
                    let full_path = full_path.join(filename);
                    
                    // Skip Option::None fields (which serde serializes to `null`):
                    let value = &self.#field_name_iter;
                    if serde_json::to_string(&value).unwrap() != "null" {

                        // Check for existing file. Do not overwrite.
                        if !full_path.exists() {
                            let mut filename = stringify!(#field_name_iter).to_string();
                            let writer = File::create_new(&full_path)?;
                            serde_json::to_writer_pretty(&writer, &self.#field_name_iter).unwrap();
                        }
                    }
                )*
                Ok(())
            }
        }
    };
    gen.into()
}