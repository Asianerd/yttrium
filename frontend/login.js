var login = true;

function toggleLoginState() {
    login = !login;
    updateLoginState();
}

function updateLoginState() {
    var inputSection = document.getElementById("input-section");
    inputSection.ariaLabel = login ? 'login' : 'signup';

    var loginPrompt = document.getElementById("login-prompt");
    loginPrompt.innerText = login ? 'dont have an account? sign up' : 'have an account? login';

    var loginButton = document.getElementById("login-button");
    loginButton.innerText = login ? 'login' : 'sign up';
}

/*
username blank
*username doesnt exist
*password incorrect


username blank
*username taken
invalid characters (spaces) (a-z, _ only)
too long (32 characters)

password blank
password no spaces
password too long (32 characters)
password must match
password must consist of a-z, 0-9 and '!@#$%^&*_|?.' only
*/

async function submitRequest() {
    var username = document.getElementById("username").value;
    var password = document.getElementById("password").value;
    var passwordConfirm = document.getElementById("password-confirm").value;
    var prompt = document.getElementById("result-prompt");

    if (!login) {
        let result = validateInfo(username, password, passwordConfirm);
        // console.log(result);
        prompt.innerHTML = result[1];
        if (!result[0]) {
            return;
        }
    } else {
        prompt.innerHTML = '';
        if (username.length <= 0) {
            prompt.innerHTML = 'username is empty';
            return;
        }
    
        if (password.length <= 0) {
            prompt.innerHTML = 'password is empty';
            return;
        }
    }

    send_request(`http://127.0.0.1:8000/user/attempt_`+(login ? 'login' : 'signup')+`/${username}/${password}`, (response) => {
        let result = JSON.parse(response);

        prompt.innerHTML = '';

        if (result[0]) {
            let now = new Date();
            (now.setTime(now.getTime() + (14 * 86400000)));
            document.cookie = `username=${username}; expires=${now.toUTCString()}; path=/`;
            // console.log("redirect occurs");
            location.href = '/frontend/index.html';
        } else {
            prompt.innerHTML = result[1];
        }
    });

    // var http = new XMLHttpRequest();
    // http.onreadystatechange = function() {
    //     if ((this.readyState == 4) && (this.status == 200)) {

    //     }
    // }

    // http.open("GET", `http://127.0.0.1:8000/user/user_exists/${username}`, true);
    // http.setRequestHeader("Content-type", "application/x-www-form-urlencoded");
    // http.send();
}

function validateInfo(name, ps, confirm_ps) {
    if ((name.length <= 0) || (name.length > 32)) {
        return [false, 'username must be 0-32 characters'];
    }

    if ((ps.length <= 0) || (ps.length > 32)) {
        return [false, 'password must be 0-32 characters'];
    }

    if (!(/^[a-zA-Z0-9_.]+$/.test(name))) {
        return [false, "username must consist of a-z, 0-9, '.' and '_' only"];
    }

    if (!(/[!@#$%^&*_|?.a-zA-Z0-9]+$/.test(ps))) {
        return [false, "password must consist of a-z, 0-9 and '!@#$%^&*_|?.' only"];
    }

    if (/([ ])/.test(ps)) {
        return [false, "password must not contain spaces"];
    }

    if (ps != confirm_ps) {
        return [false, "passwords must match"];
    }

    return [true, ""];
}

document.querySelector("#username").onkeypress = function(e) {
    return "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890_".indexOf(String.fromCharCode(e.which)) >= 0;
};

["password", "password-confirm"].forEach(element => {
    document.querySelector("#" + element).onkeypress = function(e) {
        return " ".indexOf(String.fromCharCode(e.which)) < 0;
    };
});

["username", "password", "password-confirm"].forEach(element => {
    document.querySelector("#" + element).addEventListener("keydown", function(e) {
        if (e.code === 'Enter') {
            submitRequest();
        }
    })
});

updateLoginState();

// console.log(/^[a-zA-Z0-9_.]+$/.test('teA_123123Ast'));
// console.log(/([ ])/.test('teA_123123Ast'));
// console.log(/[!@#$%^&*_|?.a-zA-Z]/.test('teA_123123Ast'));

async function send_request(url, func) {
    var http = new XMLHttpRequest();
    http.onreadystatechange = function() {
        if ((this.readyState == 4) && (this.status == 200)) {
            func(this.responseText);
        }
    }

    http.open("GET", url, true);
    http.setRequestHeader("Content-type", "application/x-www-form-urlencoded");
    http.send();
}
