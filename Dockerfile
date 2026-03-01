FROM debian:trixie

# Set the working directory inside the container
WORKDIR /app

# Copy the executable from your host machine into the container's /app directory
COPY wordle_solver /app/wordle_solver
COPY unigram_freq.csv /app/unigram_freq.csv

# Make the executable file runnable (ensure it has correct permissions)
RUN chmod +x /app/wordle_solver 

# Define the command to run the executable when the container starts
CMD ["/app/wordle_solver"]
