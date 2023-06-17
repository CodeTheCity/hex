library(tidyverse)
library(geos)
library(sf) #for reading shapefiles
library(geos) #for reducing size of shapefiles
library(rmapshaper) #for reducing size of shapefiles
library(keyring)
library(clhex)
library(geojsonio)
library(elevatr)
library(terra)
library(giscoR)
library(marmap)
library(geodata)
library(SRTM)
library(rgdal)




# Get Maps ##########################################
#
# # 1. HBs https://maps.gov.scot/ATOM/shapefiles/SG_NHS_HealthBoards_2019.zip
##  2. Datazones https://www.data.gov.uk/dataset/ab9f1f20-3b7f-4efa-9bd2-239acf63b540/data-zone-boundaries-2011
# # 3. IZs https://www.data.gov.uk/dataset/133d4983-c57d-4ded-bc59-390c962ea280/intermediate-zone-boundaries-2011
# # 4. care homes: https://data.spatialhub.scot/dataset/care_homes_for_older_people-is
# # 5. gp: https://data.spatialhub.scot/dataset/gp_practices-is
# # 6 Council https://data.spatialhub.scot/dataset/local_authority_boundaries-is
# # 7. hospitals: https://data.spatialhub.scot/dataset/nhs_hospitals-is
#  #8. pharmacy: https://data.spatialhub.scot/dataset/pharmacies-is
#  # 9. schools: https://data.spatialhub.scot/dataset/school_catchments-is
#  # 10. town centers: https://data.spatialhub.scot/dataset/town_centres-is
# # 11. Alcohol Prohibition Areas: https://data.spatialhub.scot/dataset/alcohol_prohibition_areas-is
#  #12. libraries https://data.spatialhub.scot/dataset/libraries-is/resource/570de9e0-999e-4bfd-9ed6-71b09545965e
##  13. make area maps with CreateHSCPAreaMaps.r script
##  14. wards, holyrood, council, westminster: https://osdatahub.os.uk/downloads/open/BoundaryLine
##  country/ council outlines https://gadm.org/maps/GBR/scotland/aberdeenshire.html
##  15. others: https://martinjc.github.io/UK-GeoJSON/
##
##  Spacial hub web key : d0cb7d99-1638-44ac-ba91-85914d451e55

#Open and wrangle shapefiles # dsn = data source name, then save
#
# Get data to add council to each record to aid assessment of hexmap production
load(file = "../LookupTables/data/LookupTables/Postcode_Lookup.Rda")
temp <-
  Lookup_postcode %>% select(
    ElectoralWard2022Code,
    IZcode,
    ScottishParliamentaryConstituency2021Code,
    UKParliamentaryConstituency2005Code,
    AreaCode,
    Council
  ) %>% distinct()

# Get council map
if (file.exists("./data/CouncilsMap.rds"))
{
  CouncilsMap <- readRDS(file = "./data/CouncilsMap.rds")
} else {
  CouncilsMap <- st_read(
    dsn = "./data/raw/district_borough_unitary_region.shp",
    stringsAsFactors = TRUE,
    quiet = TRUE,
    as_tibble = TRUE
  )  %>% filter(substr(CODE, 1, 1) == "S") %>%
select(Council = NAME,
                CouncilArea2019Code = CODE,
                geometry) %>%
    st_make_valid() %>% st_transform("EPSG:27700") #Transform coordinates to British National Grid format
  saveRDS(CouncilsMap, file =  "./data/CouncilsMap.rds")
}

#get holyrood map
if (file.exists("./data/HolyroodMap.rds"))
{
  HolyroodMap <- readRDS(file = "./data/HolyroodMap.rds")
} else {
    HolyroodMap <- st_read(
    dsn = "./data/raw/scotland_and_wales_const_region.shp",
    stringsAsFactors = TRUE,
    quiet = TRUE,
    as_tibble = TRUE
  ) %>% filter(AREA_CODE == "SPC") %>% select(
    ScottishParliamentaryConstituency2021Code = CODE,
    Holyrood = NAME,
    geometry
  )  %>% st_transform("EPSG:27700")
  #Transform coordinates to British National Grid format
  saveRDS(HolyroodMap, file = "./data/HolyroodMap.rds")
}

#get Westminster map
if (file.exists("./data/WestminsterMap.rds"))
{
  WestminsterMap <-  readRDS(file = "./data/WestminsterMap.rds")
} else {
  WestminsterMap <- st_read(
    dsn = "./data/raw/westminster_const_region.shp",
    stringsAsFactors = TRUE,
    quiet = TRUE,
    as_tibble = TRUE
  ) %>% filter(substr(CODE, 1, 1) == "S") %>% select(
    UKParliamentaryConstituency2005Code = CODE,
    Westminster = NAME,
    geometry
  )  %>% st_transform("EPSG:27700")
  #Transform coordinates to British National Grid format
  saveRDS(WestminsterMap, file = "./data/WestminsterMap.rds")
}

#get DZ map
if (file.exists("./data/DZMap.rds"))
{
  DZMap <-  readRDS(file = "./data/DZMap.rds")
} else {
  DZMap <- st_read(
    dsn = "./data/raw/SG_DataZone_Bdry_2011.shp",
    stringsAsFactors = TRUE,
    quiet = TRUE,
    as_tibble = TRUE
  ) %>% select(DataZone2011Code = DataZone, Datazone = Name, geometry) %>%
    st_transform("EPSG:27700") #Transform coordinates to British National Grid format
  temp <-
    Lookup_postcode %>% select(
      DataZone2011Code,
      Council
    ) %>% distinct()
  DZMap <- DZMap %>% left_join(temp, by = "DataZone2011Code")
   saveRDS(DZMap, file = "./data/DZMap.rds")
}

#get IZ map
if (file.exists("./data/IZMap.rds"))
{
  IZMap <- readRDS(file = "./data/IZMap.rds")
} else {
  IZMap <- st_read(
    dsn = "./data/raw/SG_IntermediateZone_Bdry_2011.shp",
    stringsAsFactors = TRUE,
    quiet = TRUE,
    as_tibble = TRUE
  ) %>% select(IZcode = InterZone, IZname = Name, geometry) %>%
    st_transform("EPSG:27700")
  #Transform coordinates to British National Grid format
  temp <-
    Lookup_postcode %>% select(
      IZcode,
      Council
    ) %>% distinct()
  IZMap <- IZMap %>% left_join(temp, by = "IZcode")
     saveRDS(IZMap, file = "./data/IZMap.rds")
}



#get DZ_Centres
if (file.exists("./data/DZ_Centres.rds"))
{
  DZ_Centres <-  readRDS(file = "./data/DZ_Centres.rds")
} else {
  DZ_Centres <- st_read(
    dsn = "./data/raw/SG_DataZone_Cent_2011.shp",
    stringsAsFactors = TRUE,
    quiet = TRUE,
    as_tibble = TRUE
  ) %>% select(DataZone2011Code = DataZone, Datazone = Name, geometry) %>% st_transform("EPSG:27700") #Transform coordinates to British National Grid format
  saveRDS(DZ_Centres, file = "./data/DZ_Centres.rds")
}

#get IZ_Centres
if (file.exists("./data/IZ_Centres.rds"))
{
  IZ_Centres <-  readRDS(file = "./data/IZ_Centres.rds")
} else {
  IZ_Centres <- st_read(
    dsn = "./data/raw/SG_IntermediateZone_Cent_2011.shp",
    stringsAsFactors = TRUE,
    quiet = TRUE,
    as_tibble = TRUE
  ) %>% select(IZcode = InterZone, IZname = Name, geometry)  %>% st_transform("EPSG:27700") #Transform coordinates to British National Grid format
  saveRDS(IZ_Centres, file = "./data/IZ_Centres.rds")
}

#get HBMap map
if (file.exists("./data/HBMap.rds"))
{
  HBMap <- readRDS(file = "./data/HBMap.rds")
} else {
  HBMap <- st_read(
    dsn = "./data/raw/SG_NHS_HealthBoards_2019.shp",
    stringsAsFactors = TRUE,
    quiet = TRUE,
    as_tibble = TRUE
  ) %>% select(HBCode, HBName, geometry) %>%
    st_transform("EPSG:27700") #Transform coordinates to British National Grid format
  saveRDS(HBMap, file = "./data/HBMap.rds")
}

#get WardMap map
if (file.exists("./data/WardMap.rds"))
{
  WardMap <- readRDS(file = "./data/WardMap.rds")
} else {
  WardMap <- st_read(
    dsn = "./data/raw/district_borough_unitary_ward_region.shp",
    stringsAsFactors = TRUE,
    quiet = TRUE,
    as_tibble = TRUE
  ) %>% filter((substr(CODE, 1, 1) == "S")) %>% select(ElectoralWard2022Code = CODE,
                                                       ElectoralWard2022Name = NAME,
                                                     geometry) %>% st_transform("EPSG:27700") #Transform coordinates to British National Grid format
  temp <-
    Lookup_postcode %>% select(
      ElectoralWard2022Code,
      Council
    ) %>% distinct()
  WardMap <- WardMap %>% left_join(temp, by = "ElectoralWard2022Code")

  saveRDS(WardMap, file = "./data/WardMap.rds")
}

#get Alcohol Prohibition Areas map
if (file.exists("./data/AlcProMap.rds"))
{
  AlcProMap <-  readRDS(file = "./data/AlcProMap.rds")
} else {
  AlcProMap <- st_read(
    dsn = "./data/raw/pub_apa.shp",
    stringsAsFactors = TRUE,
    quiet = TRUE,
    as_tibble = TRUE
  ) %>% st_transform("EPSG:27700") #Transform coordinates to British National Grid format
  saveRDS(AlcProMap, file = "./data/AlcProMap.rds")
}

#get Carehome map
if (file.exists("./data/Carehome.rds"))
{
  Carehome <-  readRDS(file = "./data/Carehome.rds")
} else {
  Carehome <- st_read(
    dsn = "./data/raw/pub_catr.shp",
    stringsAsFactors = TRUE,
    quiet = TRUE,
    as_tibble = TRUE
  ) %>% st_transform("EPSG:27700") #Transform coordinates to British National Grid format
  saveRDS(Carehome, file = "./data/Carehome.rds")
}

#get AssetMap map
if (file.exists("./data/AssetMap.rds"))
{
  AssetMap <-  readRDS(file = "./data/AssetMap.rds")
} else {
  AssetMap <- st_read(
    dsn = "./data/raw/pub_catr.shp",
    stringsAsFactors = TRUE,
    quiet = TRUE,
    as_tibble = TRUE
  ) %>% st_transform("EPSG:27700") #Transform coordinates to British National Grid format
  saveRDS(AssetMap, file = "./data/AssetMap.rds")
}

#get GP Practice map
if (file.exists("./data/GPMap.rds"))
{
  GPMap <- readRDS(file = "./data/GPMap.rds")
} else {
  GPMap <- st_read(
    dsn = "./data/raw/pub_gpprac.shp",
    stringsAsFactors = TRUE,
    quiet = TRUE,
    as_tibble = TRUE
  ) %>% st_transform("EPSG:27700") #Transform coordinates to British National Grid format
  saveRDS(GPMap, file = "./data/GPMap.rds")
}

#get Library map
if (file.exists("./data/LibraryMap.rds"))
{
  LibraryMap <-  readRDS(file = "./data/LibraryMap.rds")
} else {
  LibraryMap <- st_read(
    dsn = "./data/raw/pub_lib.shp",
    stringsAsFactors = TRUE,
    quiet = TRUE,
    as_tibble = TRUE
  ) %>% st_transform("EPSG:27700") #Transform coordinates to British National Grid format
  saveRDS(LibraryMap, file = "./data/LibraryMap.rds")
}

#get Hospital map
if (file.exists("./data/HospitalMap.rds"))
{
  HospitalMap <- readRDS(file = "./data/HospitalMap.rds")
} else {
  HospitalMap <- st_read(
    dsn = "./data/raw/pub_hosp.shp",
    stringsAsFactors = TRUE,
    quiet = TRUE,
    as_tibble = TRUE
  ) %>% st_transform("EPSG:27700") #Transform coordinates to British National Grid format
  saveRDS(HospitalMap, file = "./data/HospitalMap.rds")
}

#get Pharmacy map
if (file.exists("./data/PharmacyMap.rds"))
{
  PharmacyMap <- readRDS(file = "./data/PharmacyMap.rds")
} else {
  PharmacyMap <- st_read(
    dsn = "./data/raw/pub_pharm.shp",
    stringsAsFactors = TRUE,
    quiet = TRUE,
    as_tibble = TRUE
  ) %>% st_transform("EPSG:27700") #Transform coordinates to British National Grid format
  saveRDS(PharmacyMap, file = "./data/PharmacyMap.rds")
}

#get Town Centre
if (file.exists("./data/TownCenters.rds"))
{
  TownCenters <-  readRDS(file = "./data/TownCenters.rds")
} else {
  TownCenters <- st_read(
    dsn = "./data/raw/pub_townc.shp",
    stringsAsFactors = TRUE,
    quiet = TRUE,
    as_tibble = TRUE
  ) %>% st_transform("EPSG:27700") #Transform coordinates to British National Grid format
  saveRDS(TownCenters, file = "./data/TownCenters.rds")
}

#get School MAp
if (file.exists("./data/SchoolMap.rds"))
{
  SchoolMap <- readRDS(file = "./data/SchoolMap.rds")
} else {
  SchoolMap <- st_read(
    dsn = "./data/raw/pub_schlsn.shp",
    stringsAsFactors = TRUE,
    quiet = TRUE,
    as_tibble = TRUE
  ) %>% st_transform("EPSG:27700") #Transform coordinates to British National Grid format
  saveRDS(SchoolMap, file = "./data/SchoolMap.rds")
}

#Get HSCP Area Maps
if (file.exists("./data/HSCPAreaMap.rds"))
{
  HSCPAreaMap <- readRDS(file = "./data/HSCPAreaMap.rds")
} else {
  DZMap <-
    readRDS(file = "./data/DZMap.rds") %>%
    select(DataZone2011Code, geometry)
  load("../LookupTables/data/LookupTables/Postcode_lookup.Rda")
  temp <- Lookup_postcode %>%
    select(DataZone2011Code, Area, AreaCode, Council) %>% distinct()
  HSCPAreaMap <-
    left_join(DZMap, temp, by =  "DataZone2011Code") %>% select(!DataZone2011Code)
  # Combine polygons
  HSCPAreaMap <- HSCPAreaMap %>%
    group_by(Area, AreaCode, Council) %>%
    summarise() %>%
    st_cast("MULTIPOLYGON") %>%
    st_make_valid()  %>%
    st_collection_extract("POLYGON") %>% st_transform("EPSG:4326") #Transform coordinates to  WGS84 - World Geodetic System 1984, used in GPS
    saveRDS(HSCPAreaMap, file = "./data/HSCPAreaMap.rds")
}


# Get locality map
if (file.exists("./data/LocalityMap.rds"))
{
  LocalityMap <- readRDS(file = "./data/LocalityMap.rds")
} else {
  LocalityMap <- st_read(
dsn = "./data/raw/Localities2020_MHW.shp",
    stringsAsFactors = TRUE,
    quiet = TRUE,
    as_tibble = TRUE
  ) %>% st_transform("EPSG:27700") #Transform coordinates to British National Grid format
  saveRDS(LocalityMap, file = "./data/LocalityMap.rds")
}



# # Show map
# CouncilsMap %>%
#   ggplot() +
#   geom_sf(color = "steelblue",
#           alpha = 0.9) +
#   theme_void() +
#   geom_sf_text(
#     mapping = aes(label =  stringr::str_wrap(Council, 5)),
#     size = 1,
#     lineheight = 0.75
#   )
#
#
#
