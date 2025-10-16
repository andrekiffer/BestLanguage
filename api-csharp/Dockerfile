FROM mcr.microsoft.com/dotnet/aspnet:8.0 AS base
WORKDIR /app
EXPOSE 80
EXPOSE 443

FROM mcr.microsoft.com/dotnet/sdk:8.0 AS build
WORKDIR /src
COPY ["ApiCsharp.csproj", "."]
RUN dotnet restore "ApiCsharp.csproj"
COPY . .
WORKDIR "/src"
RUN dotnet build "ApiCsharp.csproj" -c Release -o /app/build

FROM build AS publish
RUN dotnet publish "ApiCsharp.csproj" -c Release -o /app/publish

FROM base AS final
WORKDIR /app
COPY --from=publish /app/publish .
ENTRYPOINT ["dotnet", "ApiCsharp.dll"]
