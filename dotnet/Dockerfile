FROM mcr.microsoft.com/dotnet/sdk:latest AS build
WORKDIR /src 

# copy csproj and restore as distinct layers
COPY /src/*.csproj . 
RUN dotnet restore

COPY . .
RUN dotnet publish \
  -c Release \
  -o /bin \
  --no-restore

FROM mcr.microsoft.com/dotnet/runtime:latest
WORKDIR /bin

COPY --from=build /bin .
ENTRYPOINT ["./mariner"]
