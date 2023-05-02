mod operation;
mod parameter;
mod property;

use operation::Operation;
use parameter::Parameter;
use property::Property;

use itertools::Itertools;
use ramhorns::Template;
use ramhorns_derive::Content;

/// OpenApiV3 code generator.
#[derive(Clone, Debug, Default)]
pub struct OpenApiV3 {
    api: openapiv3::OpenAPI,

    output_path: std::path::PathBuf,
    api_template: Vec<TemplateFile>,
    model_templates: Vec<TemplateFile>,
    supporting_templates: Vec<TemplateFile>,

    suppress_errors: bool,
}
impl OpenApiV3 {
    /// Creates a new OpenApi V3 Generator.
    pub fn new(api: openapiv3::OpenAPI, output_path: Option<std::path::PathBuf>) -> Self {
        let output_path =
            output_path.unwrap_or_else(|| std::path::Path::new("./src/autogen").to_path_buf());
        Self {
            api,
            output_path,
            api_template: vec![
                // // Actix
                TemplateFile::new(
                    std::path::Path::new(
                        "./src/v3/codegen/default/templates/actix/client/api_clients.mustache",
                    ),
                    "apis",
                    "actix/client/mod.rs",
                ),
                TemplateFile::new(
                    std::path::Path::new(
                        "./src/v3/codegen/default/templates/actix/server/handlers.mustache",
                    ),
                    "apis",
                    "actix/server/handlers.rs",
                ),
                TemplateFile::new(
                    std::path::Path::new("./src/v3/codegen/default/templates/actix/mod.mustache"),
                    "apis",
                    "actix/mod.rs",
                ),
                TemplateFile::new(
                    std::path::Path::new(
                        "./src/v3/codegen/default/templates/actix/server/api.mustache",
                    ),
                    "apis",
                    "actix/server/mod.rs",
                ),
                // // Tower-hyper
                TemplateFile::new(
                    std::path::Path::new(
                        "./src/v3/codegen/default/templates/tower-hyper/mod.mustache",
                    ),
                    "apis",
                    "tower/mod.rs",
                ),
                TemplateFile::new(
                    std::path::Path::new("./src/v3/codegen/default/templates/tower-hyper/client/api_clients.mustache"),
                    "apis",
                    "tower/client/mod.rs",
                ),
                // // Common
                TemplateFile::new(
                    std::path::Path::new("./src/v3/codegen/default/templates/mod.mustache"),
                    "apis",
                    "mod.rs",
                ),
            ],
            model_templates: vec![TemplateFile::new(
                std::path::Path::new("./src/v3/codegen/default/templates/model.mustache"),
                "models",
                ".rs",
            )],
            supporting_templates: vec![TemplateFile::new(
                std::path::Path::new("./src/v3/codegen/default/templates/model_mod.mustache"),
                "models",
                "mod.rs",
            ),
            TemplateFile::new(
                std::path::Path::new("./src/v3/codegen/default/templates/tower-hyper/client/configuration.mustache"),
                "clients/tower",
                "configuration.rs",
            ), TemplateFile::new(
                std::path::Path::new("./src/v3/codegen/default/templates/tower-hyper/client/client.mustache"),
                "clients/tower",
                "mod.rs",
            ), TemplateFile::new(
                std::path::Path::new("./src/v3/codegen/default/templates/tower-hyper/client/body.mustache"),
                "clients/tower",
                "body.rs",
            ), TemplateFile::new(
                std::path::Path::new("./src/v3/codegen/default/templates/api_mod.mustache"),
                "apis",
                "mod.rs",
            ), TemplateFile::new(
                std::path::Path::new("./src/v3/codegen/default/templates/mod_clients.mustache"),
                "clients",
                "mod.rs",
            ), TemplateFile::new(
                std::path::Path::new("./src/v3/codegen/default/templates/lib.mustache"),
                "",
                "mod.rs",
            ), TemplateFile::new(
                std::path::Path::new(
                    "./src/v3/codegen/default/templates/actix/server/api_mod.mustache",
                ),
                "apis",
                "actix_server.rs",
            )
            ],
            ..Default::default()
        }
    }
}

#[derive(Debug, Default, Clone)]
struct TemplateFile {
    template: std::path::PathBuf,
    target_folder: std::path::PathBuf,
    target_file: String,
}
impl TemplateFile {
    fn new(template: &std::path::Path, folder: &str, file: &str) -> Self {
        Self {
            template: template.to_path_buf(),
            target_folder: std::path::PathBuf::from(folder),
            target_file: file.to_string(),
        }
    }
}

#[derive(Default, Clone, Content)]
struct ApiInfoTpl {
    apis: Vec<OperationsApiTpl>,
}
#[derive(Default, Clone, Content)]
struct Apis {
    #[ramhorns(rename = "operations")]
    apis: Vec<OperationsApiTpl>,
}
#[derive(Default, Clone, Content)]
#[ramhorns(rename_all = "camelCase")]
struct SupportingTpl {
    api_info: ApiInfoTpl,
    operations: OperationsTpl,
    models: ModelTpl,
}
#[derive(Default, Clone, Content)]
#[ramhorns(rename_all = "camelCase")]
struct ModelsTpl {
    models: ModelTpl,
}
#[derive(Default, Clone, Content)]
struct ModelTpl {
    model: Vec<Property>,
}

#[derive(Content, Default, Debug, Clone)]
struct OperationsTpl {
    operation: Vec<Operation>,
}

#[derive(Default, Content, Clone, Debug)]
#[ramhorns(rename_all = "camelCase")]
pub struct OperationsApiTpl {
    classname: String,
    class_filename: String,

    operations: OperationsTpl,
}

impl OpenApiV3 {
    /// Run the OpenApi V3 Code Generator.
    pub fn run(&self, models: bool) -> Result<(), std::io::Error> {
        let models = if models { self.models()? } else { vec![] };
        let operations = self.operations()?;
        let apis = self.apis(&operations)?;

        self.ensure_templates()?;

        self.render_supporting(&models, &operations, &apis)?;
        self.render_models(&models)?;
        self.render_apis(&apis)?;

        Ok(())
    }
    fn ensure_templates(&self) -> Result<(), std::io::Error> {
        Self::ensure_path(&self.output_path, true)?;
        let templates = self
            .supporting_templates
            .iter()
            .chain(&self.api_template)
            .chain(&self.model_templates)
            .collect::<Vec<_>>();
        self.ensure_template(&templates)
    }
    fn ensure_template_path(
        &self,
        path: &std::path::Path,
        clean: bool,
    ) -> Result<(), std::io::Error> {
        let path = self.output_path.join(path);
        Self::ensure_path(&path, clean)
    }
    fn ensure_path(path: &std::path::Path, clean: bool) -> Result<(), std::io::Error> {
        if clean && path.exists() {
            if path.is_dir() {
                std::fs::remove_dir_all(path)?;
            } else {
                std::fs::remove_file(path)?;
            }
        }
        std::fs::create_dir_all(path)
    }
    fn ensure_template(&self, templates: &Vec<&TemplateFile>) -> Result<(), std::io::Error> {
        templates
            .iter()
            .map(|template| self.ensure_template_path(&template.target_folder, true))
            .collect::<Result<(), std::io::Error>>()?;
        templates
            .iter()
            .map(|template| self.ensure_template_path(&template.target_folder, false))
            .collect::<Result<(), std::io::Error>>()
    }
    fn render_supporting(
        &self,
        models: &Vec<Property>,
        operations: &Vec<Operation>,
        apis: &Vec<OperationsApiTpl>,
    ) -> Result<(), std::io::Error> {
        self.supporting_templates
            .iter()
            .map(|e| self.render_supporting_template(e, models, operations, apis))
            .collect::<Result<(), std::io::Error>>()
    }
    fn render_apis(&self, apis: &Vec<OperationsApiTpl>) -> Result<(), std::io::Error> {
        self.api_template
            .iter()
            .map(|e| self.render_template_apis(e, apis))
            .collect::<Result<(), std::io::Error>>()
    }
    fn render_models(&self, models: &Vec<Property>) -> Result<(), std::io::Error> {
        self.model_templates
            .iter()
            .map(|e| self.render_template_models(e, models))
            .collect::<Result<(), std::io::Error>>()
    }

    fn render_supporting_template(
        &self,
        template: &TemplateFile,
        models: &Vec<Property>,
        operations: &Vec<Operation>,
        apis: &Vec<OperationsApiTpl>,
    ) -> Result<(), std::io::Error> {
        let mustache = std::fs::read_to_string(&template.template)?;
        let tpl = Template::new(mustache).unwrap();

        let path = self
            .output_path
            .join(&template.target_folder)
            .join(&template.target_file);
        tpl.render_to_file(
            path,
            &SupportingTpl {
                api_info: ApiInfoTpl { apis: apis.clone() },
                operations: OperationsTpl {
                    operation: operations.clone(),
                },
                models: ModelTpl {
                    model: models.clone(),
                },
            },
        )?;

        Ok(())
    }
    fn render_template_models(
        &self,
        template: &TemplateFile,
        models: &Vec<Property>,
    ) -> Result<(), std::io::Error> {
        let mustache = std::fs::read_to_string(&template.template)?;
        let tpl = Template::new(mustache).unwrap();

        let template_path = self.output_path.join(&template.target_folder);

        // render 1 model at a time
        for model in models.iter() {
            let path = template_path.join(model.filename()).with_extension("rs");
            tpl.render_to_file(
                path,
                &ModelsTpl {
                    models: ModelTpl {
                        model: vec![model.clone()],
                    },
                },
            )?;
        }

        Ok(())
    }
    fn render_template_apis(
        &self,
        template: &TemplateFile,
        apis: &Vec<OperationsApiTpl>,
    ) -> Result<(), std::io::Error> {
        let mustache = std::fs::read_to_string(&template.template)?;
        let tpl = Template::new(mustache).unwrap();

        let template_path = self.output_path.join(&template.target_folder);

        // render 1 model at a time
        for api in apis.iter() {
            // api templates in the form:
            // $output/$target-folder/$api-classname/$target-file
            let path = template_path
                .join(api.class_filename())
                .join(&template.target_file);
            if let Some(parent) = path.parent() {
                // we already cleaned the top-level, don't do it again as we might have other templates
                // with the form $output/$target-folder/$api-classname/$any
                Self::ensure_path(parent, false)?;
            }
            tpl.render_to_file(path, api)?;
        }

        Ok(())
    }

    fn models(&self) -> Result<Vec<Property>, std::io::Error> {
        let model = self
            .api
            .components
            .as_ref()
            .unwrap()
            .schemas
            .iter()
            //.filter(|(name, _)| name.starts_with("VolumeSpec"))
            .map(|(name, ref_or)| {
                let model = self.resolve_reference_or(ref_or, None, None, Some(name));
                trace!("Model: {} => {}", name, model);
                model
            })
            .flat_map(|m| m.discovered_models().into_iter().chain(vec![m]))
            .filter(|m| m.is_model())
            .filter(|m| !m.data_type().is_empty())
            .map(Self::post_process)
            .sorted_by(|a, b| a.schema().cmp(&b.schema()))
            .dedup_by(|a, b| a.schema() == b.schema())
            .inspect(|model| debug!("Model => {}", model))
            .collect::<Vec<Property>>();

        Ok(model)
    }
    fn operations(&self) -> Result<Vec<Operation>, std::io::Error> {
        let operation = self
            .api
            .operations()
            .map(|(path, method, operation)| Operation::new(self, path, method, operation))
            .collect::<Vec<Operation>>();

        Ok(operation)
    }
    fn apis(&self, operations: &Vec<Operation>) -> Result<Vec<OperationsApiTpl>, std::io::Error> {
        let mut tags = std::collections::HashMap::<String, OperationsApiTpl>::new();
        for op in operations {
            for tag in op.tags() {
                match tags.get_mut(tag) {
                    Some(api) => {
                        api.add_op(op);
                    }
                    None => {
                        tags.insert(tag.clone(), op.into());
                    }
                }
            }
        }

        // let apis = tags
        //     .clone()
        //     .into_values()
        //     .map(|o| o.classname().to_string())
        //     .collect::<Vec<_>>();
        // debug!("apis: {:?}", apis);

        Ok(tags
            .into_values()
            .sorted_by(|l, r| l.classname().cmp(&r.classname()))
            .collect::<Vec<_>>())
    }
}

impl OpenApiV3 {
    fn missing_schema_ref(&self, reference: &str) {
        if !self.suppress_errors {
            println!("Schema reference({}) not found", reference);
        }
    }
    fn contains_schema(&self, type_: &str) -> bool {
        let contains = match &self.api.components {
            None => false,
            Some(components) => components.schemas.contains_key(type_),
        };
        trace!("Contains {} => {}", type_, contains);
        contains
    }
    fn resolve_schema_name(&self, var_name: Option<&str>, reference: &str) -> Property {
        let type_name = match reference.strip_prefix("#/components/schemas/") {
            Some(type_name) => type_name,
            None => todo!("schema not found..."),
        };
        trace!("Resolving: {:?}/{}", var_name, type_name);
        let imd = indexmap::IndexMap::new();
        let schemas = match &self.api.components {
            None => &imd,
            Some(components) => &components.schemas,
        };
        match schemas.get(type_name) {
            None => {
                panic!("Schema {} Not found!", type_name);
            }
            Some(ref_or) => self.resolve_reference_or(ref_or, None, var_name, Some(type_name)),
        }
    }
    fn resolve_schema(
        &self,
        schema: &openapiv3::Schema,
        parent: Option<&Property>,
        name: Option<&str>,
        type_: Option<&str>,
    ) -> Property {
        trace!("ResolvingSchema: {:?}/{:?}", name, type_);
        Property::from_schema(self, parent, schema, name, type_)
    }
    fn resolve_reference_or(
        &self,
        reference: &openapiv3::ReferenceOr<openapiv3::Schema>,
        parent: Option<&Property>,
        name: Option<&str>,  // parameter name, only known for object vars
        type_: Option<&str>, // type, only known when walking the component schema list
    ) -> Property {
        match reference {
            openapiv3::ReferenceOr::Reference { reference } => {
                self.resolve_schema_name(name, reference)
            }
            openapiv3::ReferenceOr::Item(schema) => {
                self.resolve_schema(schema, parent, name, type_)
            }
        }
    }
    fn resolve_reference_or_resp(
        &self,
        content: &str,
        reference: &openapiv3::ReferenceOr<openapiv3::Response>,
    ) -> Property {
        debug!("Response: {reference:?}");
        match reference {
            openapiv3::ReferenceOr::Reference { reference } => {
                self.resolve_schema_name(None, reference)
            }
            openapiv3::ReferenceOr::Item(item) => match item.content.get(content) {
                Some(media) => match &media.schema {
                    Some(schema) => self.resolve_reference_or(schema, None, None, None),
                    None => Property::default(),
                },
                None => Property::default().with_data_property(&property::PropertyDataType::Empty),
            },
        }
    }

    fn post_process(property: Property) -> Property {
        property.post_process()
    }
}

impl OperationsApiTpl {
    /// Get a reference to the api classname.
    pub fn classname(&self) -> &str {
        &self.classname
    }
    /// Get a reference to the api class filename.
    pub fn class_filename(&self) -> &str {
        &self.class_filename
    }
    /// Add the given operation.
    pub(super) fn add_op(&mut self, operation: &Operation) {
        self.operations.operation.push(operation.clone());
    }
}

impl From<&Operation> for OperationsApiTpl {
    fn from(src: &Operation) -> OperationsApiTpl {
        OperationsApiTpl {
            class_filename: src.class_filename().into(),
            classname: src.classname().into(),
            operations: OperationsTpl {
                operation: vec![src.clone()],
            },
        }
    }
}
