use super::*;

pub fn derive(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let ast: syn::DeriveInput = syn::parse_str(&s).unwrap();
    let input_type = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    let generics = &ast.generics;

    let query_lifetime = generics.params.iter().next().unwrap();
    let crate_path = syn::parse_str::<syn::Path>("ecs").unwrap();

    match ast.data {
        syn::Data::Struct(syn::DataStruct { ref fields, .. }) => {
            let field_tys: Vec<_> = fields.iter().map(|field| &field.ty).collect();
            let field_tys = &field_tys;
            let field_names: Vec<_> = fields
                .iter()
                .map(|field| field.ident.as_ref().unwrap())
                .collect();
            let field_names = &field_names;

            let expanded = quote! {
                unsafe impl#impl_generics #crate_path::Query<#query_lifetime> for #input_type#ty_generics #where_clause {
                    type Output = Self;
                    type DirectBorrows = (#(<#field_tys as #crate_path::Query<#query_lifetime>>::DirectBorrows,)*);
                    unsafe fn borrow_direct(entity: &'a #crate_path::Entity) -> Option<Self::DirectBorrows> {
                        #(let #field_names = <#field_tys as #crate_path::Query<#query_lifetime>>::borrow_direct(entity)?;)*
                        Some((#(#field_names,)*))
                    }
                    unsafe fn get_direct(entity: &'a #crate_path::Entity) -> Option<Self> {
                        #(let #field_names = <#field_tys as #crate_path::Query<#query_lifetime>>::get_direct(entity).unwrap();)*
                        Some(Self { #(#field_names,)* })
                    }
                }
            };
            expanded
        }
        _ => unimplemented!(),
    }
}
