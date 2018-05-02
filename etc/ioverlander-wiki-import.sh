#!/usr/bin/env bash
#
# This script imports iOverlander data into a imag-wiki
#
# Requirements
# ============
#
# * imag-wiki
# * imag-gps
# * imag-store
# * jq
#
# Usage
# =====
#
# Download the JSON data files from app.ioverlander.com and run them through
# this script.
#
#   ./this/script wiki-name ioverlander-file.json
#
# What it does
# ============
#
# It imports each location from the JSON data into one entry at:
#
#       <wiki_name>/<country>/<location name>_<location id>
#
# (The location id is added because names are sometimes reused)
#
# * It uses imag-gps to set the GPS data
# * It sets the header of the entry to the data it finds from the JSON object.
#   It puts all data in the "userdata.ioverlander" namespace.
#   It strips whitespace from the keys.
#   It ignores the GPS data from the JSON object though, as this was added via
#   imag-gps.
# * It uses the "description" field as entry content.
#
# Warning
# =======
#
# As the ioverlander data sometimes contains "/" in the name, some entries are
# namespaced. This has to be fixed and the linkings have to be adapted. This is
# not yet handled by this script.
#
#

WIKI_NAME="$1"
IOVERLANDER_FILE_PATH="$2"

[[ -z "$WIKI_NAME" ]]               && echo "Wiki name missing" && exit 1
[[ -z "$IOVERLANDER_FILE_PATH" ]]   && echo "JSON file missing" && exit 1

imag wiki create-wiki "$WIKI_NAME" --no-edit
echo "Created imag wiki '$WIKI_NAME'"

tobool() {
    if [[ "$1" == "Yes" ]]; then
        echo "true"
    elif [[ "$1" == "No" ]]; then
        echo "false"
    else
        echo "\"$1\""
    fi
}

object_to_vars() {
    jq -r -c 'to_entries | .[] | .key + "=\"" + (.value | tostring | gsub("\\\""; "")) + "\""'
}


# The comments in this function can be outcommented for debugging
process_object() {
    read -r json

    local location_latitude
    local location_longitude
    local location_altitude
    local location_horizontal_accuracy
    local location_vertical_accuracy
    local amenities_Open
    local amenities_Electricity
    local amenities_Wifi
    local amenities_Kitchen
    local amenities_Restaurant
    local amenities_Showers
    local amenities_Water
    local amenities_Toilets
    local amenities_Big_rig_friendly
    local amenities_Tent_friendly
    local amenities_Pet_friendly
    local id
    local name
    local description
    local date_verified
    local category_icon_path
    local category_icon_pin_path
    local country
    local category

    #echo "----------------------------------------------------------"
    #echo "JSON = $json"
    #echo "-----------"
    #echo $json | jq '.location' | object_to_vars | sed 's,^,location_,'
    #echo $json | jq '.amenities' | object_to_vars | sed 's,^,amenities_,; s, Big,_big,; s, Rig,_rig,; s, Friendly,_friendly,'
    #echo $json | jq 'del(.location)|del(.amenities)' | object_to_vars

    eval $(echo $json | jq '.location' | object_to_vars | sed 's,^,location_,')
    eval $(echo $json | jq '.amenities' | object_to_vars | sed 's,^,amenities_,; s, Big,_big,; s, Rig,_rig,; s, Friendly,_friendly,')
    eval $(echo $json | jq 'del(.location)|del(.amenities)' | object_to_vars)

    local ctry_slug=$(echo $country | sed 's, ,_,g')
    local name_slug=$(echo $name | sed 's, ,_,g')

    echo "create = $ctry_slug/${name_slug}_${id}"

    local article=$(imag wiki --wiki "$WIKI_NAME" create "$ctry_slug/${name_slug}_${id}" --no-edit --print-id || exit 1)

    echo "ARTICLE = $article"

    imag gps add --lat="$location_latitude" --long="$location_longitude" "$article" || {
        echo "GPS setting failed"
        exit 1
    }
    imag store update --id "$article"                           \
        --header                                                \
        "userdata.ioverlander.id=\"$id\""                       \
        "userdata.ioverlander.name=\"$name\""                   \
        "userdata.ioverlander.date_verified=\"$date_verified\"" \
        "userdata.ioverlander.country=\"$country\""             \
        "userdata.ioverlander.category=\"$category\""           \
        "userdata.ioverlander.amenities.open=$(tobool "$amenities_Open")"                         \
        "userdata.ioverlander.amenities.electricity=$(tobool "$amenities_Electricity")"           \
        "userdata.ioverlander.amenities.wifi=$(tobool "$amenities_Wifi")"                         \
        "userdata.ioverlander.amenities.kitchen=$(tobool "$amenities_Kitchen")"                   \
        "userdata.ioverlander.amenities.restaurant=$(tobool "$amenities_Restaurant")"             \
        "userdata.ioverlander.amenities.showers=$(tobool "$amenities_Showers")"                   \
        "userdata.ioverlander.amenities.water=$(tobool "$amenities_Water")"                       \
        "userdata.ioverlander.amenities.toilets=$(tobool "$amenities_Toilets")"                   \
        "userdata.ioverlander.amenities.big_rig_friendly=$(tobool "$amenities_Big_rig_friendly")" \
        "userdata.ioverlander.amenities.tent_friendly=$(tobool "$amenities_Tent_friendly")"       \
        "userdata.ioverlander.amenities.pet_friendly=$(tobool "$amenities_Pet_friendly")"         \
        --content \""$description\"" || {
            echo "header/content setting failed";
            exit 1
        }

}

cat "$IOVERLANDER_FILE_PATH" | jq -c '.[]' | while read line; do
    process_object
done

