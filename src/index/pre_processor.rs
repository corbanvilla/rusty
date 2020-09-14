/// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder

use super::super::ast::{ Variable, CompilationUnit,DataType,  DataTypeDeclaration};




pub fn pre_process(unit: &mut CompilationUnit) {
    
    //process all local variables from POUs
    for pou in unit.units.iter_mut() {
        let all_variables = pou.variable_blocks.iter_mut()
        .flat_map(|it| it.variables.iter_mut())
                    .filter(|it| should_generate_implicit_type(it));
                    
        for var in all_variables {
            pre_process_variable_data_type(pou.name.as_str(), var, &mut unit.types)   
        }
    }
   
    //process all variables from GVLs
    let all_variables = unit.global_vars.iter_mut()
    .flat_map(|gv| gv.variables.iter_mut())
                .filter(|it| should_generate_implicit_type(it));
                
    for var in all_variables {
        pre_process_variable_data_type("global", var, &mut unit.types)   
    }
    
}

fn should_generate_implicit_type(variable: &Variable) -> bool {
    match variable.data_type { 
        DataTypeDeclaration::DataTypeReference {..} => false,
        DataTypeDeclaration::DataTypeDefinition {..} => true,
    }
}

fn pre_process_variable_data_type(container_name: &str, variable: &mut Variable, types: &mut Vec<DataType>) {
    let new_type_name = format!("__{}_{}", container_name, variable.name);
    if let DataTypeDeclaration::DataTypeDefinition {mut data_type}  = 
    variable.replace_data_type_with_reference_to(new_type_name.clone()) {
        // create index entry
        data_type.set_name(new_type_name);
        types.push(data_type);
    }
    //make sure it gets generated
    
}