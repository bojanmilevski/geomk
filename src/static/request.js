async function mapCoordinates(coordinates) {
  if (coordinates.length == 0) {
    return;
  }

  for (let i = 0; i < coordinates.length; i++) {
    var lat = coordinates[i].lat;
    var lon = coordinates[i].lon;
    L.marker([lat, lon]).addTo(map)
  }
}

async function sendRequest() {
  const City = document.getElementById('city').value;
  const Query = document.getElementById('query').value;

  const response = await fetch('http://localhost:8000/request', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify({ city: City, query: Query }),
  });

  const coordinates = await response.json();
  await mapCoordinates(coordinates);
}
