# Use an official Microsoft Windows Server Core image as the base
# Other options include nanoserver depending on app compatibility
#FROM ://mcr.microsoft.com
FROM mcr.microsoft.com

# Set the working directory inside the container
WORKDIR /

# Copy the executable and any required files from your host to the container
COPY wordle_solver.exe .
COPY unigram_freq.csv .
# Add other dependencies as needed

# Define the command to run the executable when the container starts
CMD ["wordle_solver.exe"]
