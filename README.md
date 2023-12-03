# GEOMK

GeoMK is a web server written in Rust that queries [OpenStreetMap](https://www.openstreetmap.org)
through the [Overpass API](https://overpass-turbo.eu) and retrieves coordinates based on user
queries.

## EXAMPLE

![Example video](./assets/example_video.mkv)

## DESCRIPTION

GeoMK provides a streamlined and customizable solution for users to interact with OpenStreetMap
data via the [Overpass API](https://overpass-turbo.eu/). This tool enables users to effortlessly
retrieve information about specific items within a designated city. By default, the program
retrieves all drinking water taps in Skopje, Macedonia that have been pinned by other
OpenStreetMap users. The program encapsulates the complexity of query formatting, allowing users
to tailor their search by specifying the item and city of interest through two input fields.

During design and implementation discussions, a unanimous agreement was settled between the team
members. We agreed that each time the program is started, it queries Overpass so that each time the
user is served the latest data. Although it is a taxing and heavy operation, we ultimately decided
that relevant and correct information outweighs performance and effectiveness.

Upon initiation, the program initializes a server, which serves a map and 2 options for the user.
After the user makes a choice, the server orchestrates a sequence of operations to fetch and process
data. It starts by sending pre-coded queries to the Overpass API, which are completely abstracted
from the user. The response is a JSON string containing relevant coordinates for the queried item.
The program extracts and transforms this data into structured objects and queries the API for the
boundaries of the user-specified city, storing this data alongside the item coordinates in a
dedicated database.

The program manages database operations, creating tables for both the queried item's coordinates
and the city's boundaries. The data is then inserted into their respective tables, establishing a
comprehensive and organized dataset for subsequent analysis. To ensure usability, the server
employs the **pipe-and-filter** design pattern, creating a modular and extensible architecture for
data processing.

Upon completion of the data handling pipeline, the server reads the information from the database
and applies a filter to retain only the coordinates relevant to the user-specified city. The final
set of filtered coordinates is presented to the user in the terminal, offering a concise and
tailored output based on their search criteria.

## REQUIREMENTS

### FUNCTIONAL

- Query Customization:

  - Users can specify the item to query.
  - Users can select the city for the search.

- Data Retrieval and Transformation:

  - The program must effectively send and handle queries to the Overpass API.
  - Overpass API responses (JSON) must be parsed and transformed into usable objects.

- Database Management:

  - The program must create a database.
  - A database table must be created for each city.
  - A database table must be created all the queried item's coordinates.
  - Data retrieved from the Overpass API must be appropriately inserted into the corresponding
    tables.

- Pipe-and-Filter Design Pattern:

  - The program must implement a pipe-and-filter design pattern to process and filter data.

- Data Display:
  - Filtered coordinates must be marked on the visible map.

### NON-FUNCTIONAL

- Performance

  - The program should handle data retrieval and transformation efficiently, minimizing latency.
  - Database operations should be optimized for speed.

- Reliability:

  - The program must handle errors gracefully, providing informative error messages to users.
  - Database operations should be robust and resistant to data corruption.

- Scalability:

  - The program should be designed to handle a growing dataset, ensuring performance scales with
    increased data volume.

- Usability:

  - The web interface should be user-friendly, providing clear instructions and feedback.

- Security:

  - The program should handle user input securely to prevent potential vulnerabilities.
  - Access to the database should be restricted appropriately.

- Maintainability:

  - Code should be well-documented, following Rust best practices.
  - Changes to Overpass API endpoints or query formats should be easily accommodated.

- Portability:
  - The program should be platform-independent, running seamlessly on various operating systems
    where Rust is supported.

## ARCHITECTURAL DESIGN

### CONCEPTUAL VIEW

The conceptual view of the service shows how the UI communicates with the user manager, and the
coordinates service, which both get their information from the database. After the information
has been queried from the database, the UI gets updated.

![Conceptual design image](./assets/conceptual.png)

### EXECUTION

The execution design shows the service during its runtime. The service sends asynchronous calls to
the coordinates and user manager service. Both of these services communicate with external systems.

![Execution design image](./assets/execution_1.png)

If we were to trace the service's execution:

1. User submits their request.
2. User manager authenticates the user.
3. Query parameters get passed to the Overpass API (abstracted).
4. Relevant information gets written into the database.
5. Coordinates go through a pipe-and-filter.
6. Information is returned to the user.
7. Coordinates are mapped on the UI.

Here is an image showing the flow of execution during runtime:

![Execution flow image](./assets/execution_2.png)

### IMPLEMENTATION

The implementation architecture goes in great depth about the system's implemented technologies
and functionalities.

The system displays React components to the user. Users interact with the service through these
components. The HTTP server is responsible for retrieving information from the server to the user.
Below, we will wee how the server is implemented, what technologies we use and how these
technologies communicate with each other.

![Implementation image](./assets/implementation_1.png)

[Axum](https://github.com/tokio-rs/axum) uses `tower` and `hyper` under the hood to manage web
requests in a streamlined fashion.

- Information is retrieved from Overpass.

- All information gets written to a local SQLite database.

![Implementation image](./assets/implementation_2.png)

A detailed explanation of the server's execution after a user sends a query:

![Sequential diagram](./assets/sequential.png)

Note the pipe-and-filter design pattern, which sends the coordinates down a pipe, runs filters on
the coordinates, and returns a new list of coordinates (possibly reduces), so that they match the
user's initial queries.

## GUI MOCKUP

A simple mockup image of GeoMK's frontend:

![GUI mockup image](./assets/gui_mockup.png)

## LICENSE

This software is licensed under the [GPL v3.0 License](https://www.gnu.org/licenses/gpl-3.0.en.html).

## LIVE DOCUMENTATION

As this repository grows, the documentation changes along with it - providing information that is
up to date and relevant to the state of this project.

## CONTRIBUTORS

- Bojan Milevski, 211561
- Kristijan Selchanec, 211543
- Damjan Gjorgjievski, 211538
- Marko Zafirovski, 211274
- Nikola Ivanovski, 216127
