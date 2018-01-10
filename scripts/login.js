function submit_login() {
  var pass = document.getElementById('password').value;
  var email = document.getElementById('email').value;
  scrypt(pass, email, { N: 4, r: 4, p: 2, encoding: 'hex' }, function(hash) {
    console.log('the hash of ' + pass + ' is ' + hash + '.');
    document.getElementById('pass_hash').value = hash;
    console.log(document.getElementById('login_form'));
    document.getElementById('login_form').submit(); //form submission
  });
}
