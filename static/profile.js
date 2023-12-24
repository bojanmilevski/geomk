const getCoordinates = async () => {
	fetch('/api/get')
	.then(response => {
		if (!response.ok) {
			throw Error('get pins error');
		} return response.json();
	}).then(data => {
		insertCoordinates(data);
	}).catch(error => {
		alert(error.message);
		return;
	});
}

const insertCoordinates = async (coordinates) => {
	const div = document.getElementById('coordinates');

	while (div.firstChild) {
		div.removeChild(div.firstChild);
	}

	coordinates.forEach(c => {
		const p = document.createElement('p');
		p.textContent = `lat: ${c.lat} lon: ${c.long}`;
		div.appendChild(p);
	})
}
