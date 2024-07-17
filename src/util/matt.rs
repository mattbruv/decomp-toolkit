use itertools::{structs, Itertools};

use super::dwarf::{
    process_structure_tag, AttributeKind, AttributeValue, DwarfInfo, StructureBase, StructureType,
};

pub fn get_struct_from_base(info: &DwarfInfo, base: &StructureBase) -> Option<StructureType> {
    if let Some(base_name) = &base.name.clone() {
        let struct_tag = info //
            .tags //
            .iter()
            .find(|(idx, tag)| {
                return tag.attributes.iter().any(|attr| {
                    attr.kind == AttributeKind::Name
                        && match &attr.value {
                            AttributeValue::String(s) => s == base_name,
                            _ => false,
                        }
                });
            });

        // we've found the struct, parse and return it
        if let Some(fuck) = struct_tag {
            let the_fucking_struct = process_structure_tag(info, fuck.1);
            if let Ok(s) = the_fucking_struct {
                return Some(s);
            }
        }
    }
    None
}

pub fn get_structs_recursive(info: &DwarfInfo, the_struct: StructureType) -> Vec<StructureType> {
    let mut structs = vec![the_struct.clone()];

    //
    let base_structs: Vec<StructureType> = the_struct
        .bases
        .iter() //
        .flat_map(|x| get_struct_from_base(info, x))
        .collect_vec();

    for base in base_structs {
        structs.extend(get_structs_recursive(info, base.clone()))
    }

    structs
}
