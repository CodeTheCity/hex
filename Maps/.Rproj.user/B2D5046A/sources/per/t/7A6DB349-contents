library(tidyverse)
theme_set(theme_minimal())
library(sf)
library(gridExtra)

Grid <- st_read("./data/geojson/AberdeenshireWards.geojson")

Council<-"Aberdeenshire"

# Grid <- Grid %>% filter(Council=="Aberdeenshire")


p1<-Grid %>%
  ggplot() +
  geom_sf(aes(fill=Council),
          color = "steelblue",
          alpha = 0.9,
          show.legend = FALSE) +
  theme_void() +
  geom_sf_text(
    mapping = aes(label =  stringr::str_wrap(ElectoralWard2022Name, 5)),
    size = 1.5,
    lineheight = 0.75
  )



Grid <- readRDS("./data/WardMap.rds")

Grid <- Grid %>% filter(Council=="Aberdeenshire")


p2<-Grid %>%
  ggplot() +
  geom_sf(aes(fill=Council),
          color = "steelblue",
          alpha = 0.9,
          show.legend = FALSE) +
  theme_void() +
  geom_sf_text(
    mapping = aes(label =  stringr::str_wrap(ElectoralWard2022Name, 5)),
    size = 1.5,
    lineheight = 0.75
  )

grid.arrange(p1, p2, ncol=2)
