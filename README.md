# Granium Assignment

## Instructions

### 1. Clone the Repository

To get started with the project, follow these steps to clone the repository to your local directory:

1. Open your terminal.

2. Navigate to the directory where you want to clone the repository.

3. Run the following command:

```bash
git clone https://github.com/DeLion13/granium_test.git
```

4. Once the cloning process is complete, you will have a local copy of the repository in your chosen directory.

### 2. Run Docker Daemon

Ensure that the Docker daemon is running on your system.

### 3. Enter the Cloned Repository

Navigate into the cloned repository directory:

```bash
cd granium_test
```

### 4. Build Docker Image

Build the Docker image using the following command:

```bash
docker build -t granium .
```

### 5. Run Docker Container

Run the Docker container using the following command:

```bash
docker run -p 8080:8080 granium
```

### 6. Test the Application

Once the container is running, test the application by sending a GET request to the `/api/v1/ltp` endpoint:

```bash
curl http://localhost:8080/api/v1/ltp | jq
```

This command sends a GET request to the specified endpoint and pipes the output to `jq` for pretty printing.

Expected behaviour is:

```json
{
  "ltp": [
    {
      "pair": "BTC/USD",
      "amount": "67109.70000"
    },
    {
      "pair": "BTC/EUR",
      "amount": "62693.90000"
    },
    {
      "pair": "BTC/CHF",
      "amount": "60900.00000"
    }
  ]
}
```

## Instructions

To run tests on your local environment feel free to use this command:

```bash
cargo test --package granium --bin granium --all-features -- test::test --show-output
```

