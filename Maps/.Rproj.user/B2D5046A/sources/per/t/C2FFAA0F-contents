# Script to display data on hexbin plot
##################
library(tidyverse)
library(geojsonio) # Required for geojson_read
library(broom) # for tidy function
library(rgeos) # Calculate the centroid of each hexagon
library(viridisLite) # for colour palette
library(png) # for readPNG
library(bbplot)

# Prep logo
logo <-  patchwork::inset_element(
  p =  readPNG(
    "../CommonData/ADPLogo.png",
    native = TRUE,
    info = TRUE
  ),
  left = 0.91,
  bottom = 0.92,
  right = .98,
  top = 0.98,
  align_to = "full",
  clip = TRUE,
  ignore_tag = TRUE
)

#Create Theme for HexBin Plots
theme_hexbin <- function() {
  font <- "Roboto"

  theme(
    aspect.ratio = 1,
    text = element_text(
      family = font,
      size = 14 ,
      color = "black"
    ),
    rect = element_rect(fill = "transparent"),
    legend.background = element_blank(),
    legend.key = element_blank(),
    legend.key.height =  unit(3, units = "mm"),
    legend.position = "bottom",
    legend.direction = "horizontal",
    legend.title.align = 0.5,
    legend.justification = "centre",
    legend.title =  element_text(
      size = 12 ,
      color = "black",
      margin = margin(b = 0.1,
                      t = 0,
                      unit = "cm")
    ),
    legend.text =   element_text(
      size = 10 ,
      color = "black",
      margin = margin(b = 0.1,
                      t = 0.1,
                      unit = "cm")
    ),
    axis.title = element_blank(),
    axis.text = element_blank(),
    axis.ticks = element_blank(),
    axis.line = element_blank(),

    panel.grid.major = element_blank(),
    panel.grid.minor = element_blank(),
    panel.border = element_rect(color = "transparent", fill = "transparent"),
    panel.background =  element_rect(color = "transparent", fill = "transparent"),
    panel.spacing = unit(0.5, "cm"),
    strip.background =  element_blank(),
    strip.text.x = element_text(
      size = 12 ,
      face = "bold",
      colour = "black"
    ),
    plot.title = element_text(
      hjust = 0,
      size = 20 ,
      color = "#EC0108"
    ),
    plot.title.position = "plot",
    plot.subtitle = element_text(
      hjust = 0,
      size = 16 ,
      margin = margin(b = 0.4,
                      t = 0.0,
                      unit = "cm")
    ),
    plot.background = element_rect(fill = "#f0f0f0"),
    plot.tag.position = "bottomleft",
    plot.margin = unit(c(0.5, 0.5, 0.5, 0.5), "cm"),
    plot.caption = element_text(
      hjust = 1,
      size = 10 ,
      margin = margin(b = 0.4,
                      t = 0.2,
                      unit = "cm")
    ),
    plot.caption.position = "plot",
    complete = TRUE
  )


}

# Get Aberdeenshire IZ Hexbin
ShireHexbin <-
  geojson_read("../Maps/data/geojson/AberdeenshireIZs.geojson",  what = "sp")
##################################
#'Fortify' data for use in ggplot2
ShireHexbin_fortified <-
  tidy(ShireHexbin, region = "name") %>% distinct()

# Define centre and name each hexagon
BinCent <-
  cbind.data.frame(data.frame(
    gCentroid(ShireHexbin, byid = TRUE,  id = ShireHexbin@polygons$ID)
  ))
BinCent$IZname <- ShireHexbin@data$name


OurPeople_Rate_IZ <-
  readRDS(file = "./RecentResults/Rate_IZname.rds")

# Filter outcome data
Outcome <-
  OurPeople_Rate_IZ  %>% ungroup() %>% dplyr::select(c(level, Sex, ClientsPer100Pop))

# Expand Outcome table so there is an entry for every combination of IZ and gender
Outcome <-
  complete(Outcome, Sex, level, fill = list(ClientsPer100Pop = 0))

# Insert outcome data into Hexbin
ShireHexbin_fortified <- ShireHexbin_fortified %>%
  left_join(Outcome, by = c("id" = "level"))

# Define how values will be collated into 'bin' groups
ShireHexbin_fortified$bin <-
  cut(
    ShireHexbin_fortified$ClientsPer100Pop,
    breaks = c(-1, 0, 0.1, 0.5, 1, 2, 3, 5, 100),
    labels = c(
      "Zero",
      ">0-0.1%",
      ">0.1-0.5%",
      ">0.5-1%",
      ">1-2%",
      ">2-3%",
      ">3-5%",
      ">5%"
    ),
    include.lowest = TRUE,
    include.highest = TRUE
  )

# merge bincent and hexbin to enable easier ggplot manipulations later
ShireHexbin_fortifiedx <-
  left_join(ShireHexbin_fortified, BinCent, by = c("id" = "IZname"))
ShireHexbin_fortifiedx$id <-
  str_wrap(ShireHexbin_fortifiedx$id ,  5)


# Create colour scale
my_palette <- plasma(8, direction = -1)

# Plot Aberdeenshire Hexbins
p <- ggplot(data = ShireHexbin_fortifiedx)  +
  theme_hexbin() +
  geom_polygon(aes(
    fill = bin,
    x = long,
    y = lat,
    group = group
  ),
  color = "steelblue",
  alpha = 0.9) +
  geom_text(
    size = unit(2.2, "pt"),
    lineheight = 0.6,
    mapping = aes(
      x = x,
      y = y,
      label = id,
      colour = ClientsPer100Pop > 3
    )
  ) +

  scale_color_manual(
    values = c("black", "white"),
    aesthetics = "colour",
    na.value = "transparent",
    guide = "none"
  ) +


  scale_fill_manual(
    values = my_palette,
    na.value = "transparent",
    guide = guide_legend(
      label.position = "bottom",
      title.position = 'top',
      nrow = 1
    )
  ) +
  labs(
    title = "Hexbin Map of ADP Activity",
    subtitle = "by Intermediate Zone",
    fill = "Percentage of IZ population genders engaged with ADP services",
    caption = "Source: DAISy",
    tag = waiver(),
    alt = "Hexbin map showing each Intermediate Zone as a hexagon and coloured according to the percentage of the local population engaged with ADP services."
  ) +
  facet_wrap(vars(Sex), nrow = 1, drop = TRUE)

p





p + logo
