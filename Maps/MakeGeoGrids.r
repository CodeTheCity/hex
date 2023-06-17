# # https://rpubs.com/langton_/worksheet-extras-03
# Purpose: Convert national maps into GeoGrids

# Strategy: create hexbin map of Scottish maps
#   - Get map, tidy, transform to BNG.
#   - Create hexgrid
#   - Allocate map zones to hexgrid (computationally expensive with the
#     origional geogrid::assign_polygons.
#     Used an adaption of https://gis.stackexchange.com/a/313326 instead. )
#
options("rgdal_show_exportToProj4_warnings" = "none") # needed to hide warnings

# Libraries ####################################################################
library(tidyverse)
library(geogrid) # needed for calculate_grid and assign_polygon
library(rgeos) # required to calculate centroids
library(sp) # needed for calculating distance and converting between spacialpolygon & sf
library(clue) # needed to Solve the linear sum assignment problem
library(sf) # used to fortify between SpatialPolygonsDataFrame and sf
library(broom) # used to fortify
library(rmapshaper) # used to cleanup shape files

# library(viridis) # needed for palette colours
# library(geojson)
# library(geos)# Function #####################################################################
# create alternative function to implement grid allocation
MakeHexMap <- function(sdf, seed = 999) {

  grid_type = 'hexagonal'
  sdf$ORIG_ID = 1:nrow(sdf)
  sdf <- sdf %>% ms_simplify()

  # Origional area centres
  OrigionalAreaCenters = gCentroid(spgeom = methods::as(object = sdf, Class = "Spatial"),
                                   byid = T)

  # Calculate hexagonal grid that preserves the original geography of
  # the SpatialPolygonsDataFrame
  HexGridCells <- calculate_grid(
    shape = sdf,
    grid_type = grid_type,
    learning_rate = 0.005,
    seed = seed
  )
  HexGridCellCenters <- as.data.frame(HexGridCells[[1]])
  HexGridCellCenters$ID <-
    paste0("ID", seq.int(nrow(HexGridCellCenters)))
  HexGridCellPolygons <- HexGridCells[[2]] #SpatialPolygons
  HexGridCellPolygonIDs <-
    sapply(slot(HexGridCellPolygons, "polygons"), function(x)
      slot(x, "ID")) # get HexGrid polygon IDs

  # calculate matrix of distances between original centres and hexgrid centres
  distmatrix <- spDists(HexGridCells[[1]], OrigionalAreaCenters)

  distmatrix <- round(distmatrix, 0)

  # Solve the linear sum assignment problem using the 'Hungarian method' to find an optimal assignment of hexgrid centers to original centres.
  # Trying different approaches now:
  AssignmentSolution <- solve_LSAP(distmatrix)
  # AssignmentSolution <- adagio::assignment(distmatrix)
  # AssignmentSolution <- lp.assign(distmatrix)
  # AssignmentSolution <- LAPJV(distmatrix)
  grid_sdf = SpatialPolygonsDataFrame(
    HexGridCellPolygons,
    data.frame(row.names = HexGridCellPolygonIDs,
               ORIG_ID = AssignmentSolution[seq_along(AssignmentSolution)])
  )
  stopifnot(nrow(grid_sdf) == nrow(sdf))

  #Fortify
  grid_sdf <- st_as_sf(grid_sdf)

  # Merge in Names
  data <- st_drop_geometry(sdf)
  map <- merge(grid_sdf, data, by = "ORIG_ID") %>% select(!ORIG_ID)
  # map <- merge(grid_sdf, data, by.x = "ORIG_ID.matching", by.y = "ORIG_ID") %>% select(!ORIG_ID.matching)
  return(map)
} #end of function

# Create empty hexgrids ########################################################
#
data <- readRDS("./data/IZMap.rds")

i = 8650
data <- data %>% filter(Council== "Aberdeenshire")
map <- MakeHexMap(data, i)


ggplot(map) +
  geom_sf(aes(fill = "white"), color = "steelblue",
          show.legend = FALSE) +
  theme_void() +
  geom_sf_text(aes(label = str_wrap(IZname, width = 14)),
               size = 1.5,
               lineheight = 0.75)

 st_write(map, "./data/geojson/AberdeenshireIZHexMap.geojson")
