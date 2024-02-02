const checkFields = async (fields) => {
	fields.forEach(field => {
		if (field === '') {
			throw Error('field cannot be empty');
		}
	});
}

const signUp = async () => {
	const username = document.getElementById('username').value;
	const password = document.getElementById('password').value;
	const confirmPassword = document.getElementById('confirmPassword').value;

	try {
		await checkFields([username, password, confirmPassword]);
	} catch (error) {
		alert(error.message);
		return;
	}

	if (password !== confirmPassword) {
		alert('Passwords do not match!');
		return;
	}

	const credentials = {
		username: username,
		password: password,
	};

	try {
		await sendSignUpRequest(credentials);
		await sendLogInRequest(credentials);
	} catch (error) {
		alert(error.message);
		return;
	}
}

const sendSignUpRequest = async (credentials) => {
	const response = await fetch('/api/signup', {
		method: 'POST',
		headers: {
			'Content-Type': 'application/json',
		},
		body: JSON.stringify(credentials),
	});

	if (!response.ok) {
		throw Error('A user with that username already exists!')
	}
}

const logIn = async () => {
	const username = document.getElementById('username').value;
	const password = document.getElementById('password').value;

	const credentials = {
		username: username,
		password: password,
	};

	try {
		await checkFields([username, password]);
	} catch (error) {
		alert(error.message);
		return;
	}

	try {
		await sendLogInRequest(credentials);
	} catch (error) {
		alert(error.message);
		return;
	}
}

const sendLogInRequest = async (credentials) => {
	const response = await fetch('/api/login', {
		method: 'POST',
		headers: {
			'Content-Type': 'application/json',
		},
		body: JSON.stringify(credentials),
	});

	if (!response.ok) {
		throw Error('Incorrect password or user does not exist! Please try again!');
	}

	window.location.href = '/map.html';
	localStorage.setItem('isLoggedIn', true);
}
