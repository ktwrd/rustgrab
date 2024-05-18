const fs = require('fs');

var content = '';
var matchContent = '';
var available = [];
for (let item of fs.readdirSync('.')) {
    if (item.endsWith('.yml') == false)
    {
        continue;
    }
    let localeName = item.split('.')[0];
    available.push(localeName);
    let constName = localeName.toUpperCase();
    content += `pub(crate) const ${constName}: &str = include_str!("../lang/${item}");\n`;
    matchContent += `        "${localeName}" => crate::locale_content::${constName},\n`;
}
matchContent =
'pub fn get_content(code: String) -> String {\n' +
'    match code.as_str() {\n' +
matchContent +
'        _ => crate::locale_content::EN\n' +
'    }.to_string()\n' +
'}';

content =
content +
'\r\n' +
matchContent +
'\n\n' +
`pub(crate) const AVAILABLE: &'static [&'static str] = &[${available.map(v => '"' + v + '"').join(', ')}];`;

fs.writeFileSync("../src/locale_content.rs", content);