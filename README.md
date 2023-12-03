# GEOMK

GeoMK is a CLI program written purely in Rust that queries [OpenStreetMap](https://www.openstreetmap.org)
through the [Overpass API](https://overpass-turbo.eu) and retrieves coordinates based on user queries.

## EXAMPLE

![Example video](./assets/example.mkv)

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
  - Users can define the name of the database.
  - Users can select the city for the search.

- Data Retrieval and Transformation:

  - The program must effectively send and handle queries to the Overpass API.
  - Overpass API responses (JSON) must be parsed and transformed into usable objects.

- Database Management:

  - The program must create a database with user-specified or default name.
  - Two tables, one for item coordinates and one for city boundaries, must be created in the
    database.
  - Data retrieved from the Overpass API must be appropriately inserted into the corresponding
    tables.

- Pipe-and-Filter Design Pattern:

  - The program must implement a pipe-and-filter design pattern to process and filter data.

- Data Display:
  - Filtered coordinates must be printed to the terminal.

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

  - The CLI interface should be user-friendly, providing clear instructions and feedback.
  - Default values for queries and database parameters should be sensible and well-documented.

- Security:

  - The program should handle user input securely to prevent potential vulnerabilities.
  - Access to the database should be restricted appropriately.

- Maintainability:

  - Code should be well-documented, following Rust best practices.
  - Changes to Overpass API endpoints or query formats should be easily accommodated.

- Portability:
  - The program should be platform-independent, running seamlessly on various operating systems
    where Rust is supported.

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
