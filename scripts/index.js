var fs = require('fs');
var Handlebars = require("handlebars");

const jsProtocol = require('devtools-protocol/json/js_protocol.json');
const browserProtocol = require('devtools-protocol/json/browser_protocol.json');
const protocolDomains = jsProtocol.domains.concat(browserProtocol.domains);
const modSource = fs.readFileSync("./mod.hbs", {encoding:'utf8', flag:'r'});
const modTemplate = Handlebars.compile(modSource);
const domainSource = fs.readFileSync("./domain.hbs", {encoding:'utf8', flag:'r'});
const domainTemplate = Handlebars.compile(domainSource);

Handlebars.registerHelper('lowercase', (str) => {
  if(str && typeof str === "string") {
    return str.toLowerCase();
  }
  return '';
});

const underscore = (str) => {
  if(str && typeof str === "string") {
    str = str.replaceAll(/::/g, '/');
    str = str.replaceAll(/([A-Z]+)([A-Z][a-z])/g,'$1_$2');
    str = str.replaceAll(/([a-z\d])([A-Z])/g,'$1_$2');
    str = str.replaceAll("-", "_");
    return str.toLowerCase();
  }
  return '';
};

Handlebars.registerHelper('underscore', underscore);

Handlebars.registerHelper('struct_key', (struct, str) => {
  if (typeof struct != 'string') return '';
  if (typeof str != 'string') return '';
  str = underscore(str);
  if (str == "type") {
    str = underscore(struct) + '_' + str;
  }
  return str;
});

Handlebars.registerHelper('ufirst', (str) => {
  if (typeof str !== 'string') return '';
  return str.charAt(0).toUpperCase() + str.slice(1);
});

Handlebars.registerHelper('eq', (a, b) => {
  return a == b;
});

Handlebars.registerHelper('struct_keys', (struct, properties) => {
  if (properties == undefined) {
    return "//TODO: generic hashmap";
  }

  properties.forEach(property => {
    if (property.enum) {
      return "enum";
    }
  });
});

(() => {

  fs.mkdir("../src/protocol/", {recursive: true}, (err) => {
    if (err) {
      console.log(err);
      process.exit(1);
    }
  });

  fs.writeFile("../src/protocol/mod.rs", modTemplate({"domains": protocolDomains}), (err) => {
    if (err) {
     console.log(err);
     process.exit(1);
    }
  });

  protocolDomains.forEach(domain => {
    console.log(domain.domain);
    //var refs = [];
    //domain.types.forEach(type => {
    //  if (type.type != "object") { return; }

    //  type.properties.forEach(property => {
    //  });
    //});

    fs.writeFile("../src/protocol/" + underscore(domain.domain) + ".rs", domainTemplate({"domain": domain}), (err) => {
      if (err) {
        console.log(err);
        process.exit(1);
      }
    });
  });
})();
