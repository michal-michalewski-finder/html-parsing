const fs = require('fs');
const cheerio = require('cheerio');


for(let i=0; i< 1000; i++) {

    const start = new Date();

    const html = fs.readFileSync('./src/test.html');

    const $ = cheerio.load(html);
    const links = $('[rel="stylesheet"]:not([href*="admin-bar"])');
    console.log(links);

    const end = new Date();

    console.log('Iteration %o, %o', (end.getTime() - start.getTime()) / 1000, i);
}


