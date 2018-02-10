const express = require('express');
const app = express();

app.get('/', function (req, res) {
  res.redirect('/rusty-firefox.html');
});

app.use(express.static('node_modules/reveal.js'));
app.use(express.static('public/slides'));
// app.use(express.static('public/img'));

app.listen(9999, () => console.log('Presentation running on port 9999'));