const {
  quicktype,
  InputData,
  jsonInputForTargetLanguage,
  JSONSchemaInput,
  JSONSchemaStore,
} = require("quicktype-core");
const $RefParser = require("@apidevtools/json-schema-ref-parser");
var fs = require('fs');

function writeFile(fileName, data){
  fs.writeFile(fileName, data, function (err) {
    if (err) throw err;
      console.log('Successfully generated '+fileName);
  });
}

async function quicktypeJSONSchema(targetLanguage, typeName, jsonSchemaString) {
  const schemaInput = new JSONSchemaInput(new JSONSchemaStore());

  await schemaInput.addSource({ name: typeName, schema: jsonSchemaString });
  const inputData = new InputData();
  inputData.addInput(schemaInput);
  return await quicktype({
    inputData,
    lang: targetLanguage,
  });
}

function dereference(parentStructName, mySchema, fileName){
  $RefParser.dereference(mySchema, (err, schema) => {
    if (err) {
      console.error(err);
    }
    else {
      schemaGen = JSON.stringify(schema);
      generateSchema(parentStructName, JSON.stringify(schema), fileName);
    }
  })
}

async function generateSchema(parentStructName ,schema, fileName) {
   const { lines: rustSpec } = await quicktypeJSONSchema(
    "rust",
    parentStructName, 
    schema
  );
  //console.log(rustSpec);
  writeFile(fileName, rustSpec.join("\n"));
}

function generator(parentStructName ,base, entryPoint, outputPath){
  var rootPath = process.cwd();
  process.chdir(base);
  const myschema = require(base+"/"+entryPoint);
  dereference(parentStructName ,myschema, rootPath+"/"+outputPath);
  process.chdir(rootPath);
}

generator('Spec','./runtime-spec/schema', 'config-schema.json', 'src/runtime/mod.rs');
generator('ImageSpec','./image-spec/schema', 'config-schema.json', 'src/image/mod.rs');
