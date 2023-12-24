const getCoordinates = async () => {
	fetch('/api/get')
	.then(response => {
		if (!response.ok) {
			throw Error('get pins error');
		} return response.json();
	}).then(data => {
		insertCoordinates(data.elements);
	}).catch(error => {
		alert(error.message);
		return;
	});
}

const insertCoordinates = async (coordinates) => {
	const div = document.getElementById('coordinates');
	div.innerHTML = '';

	while (div.firstChild) {
		div.removeChild(div.firstChild);
	}

	coordinates.forEach(c => {
		const sec = document.createElement('div');

		const p = document.createElement('p');
		p.id = c.id;
		p.textContent = `lat: ${c.lat} lon: ${c.lon}`;

		const button = document.createElement('button');
		button.addEventListener('click', () => {
			deleteCoordinate(c.id);
		})
		button.innerHTML = 'Delete';

		sec.appendChild(p);
		sec.appendChild(button);
		div.appendChild(sec);
	});
}

const deleteCoordinate = async (id) => {
	await fetch('/api/delete/' + id, {
		method: 'DELETE',
	// }).then(response => {
	// 	if (!response.ok) {
	// 		throw Error('get pins error');
	// 	}
	}).then(_ => {
		getCoordinates();
	}).catch(error => {
		alert(error.message);
		return;
	});
}

getCoordinates()
