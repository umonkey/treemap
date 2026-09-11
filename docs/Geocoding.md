# Geocoding

To automatically find the street address for a tree, we use [Nominatim](https://nominatim.org/) at zoom=16. Nominatim only returns building numbers at zoom>=18, but at that zoom it often takes the street name from the closest bigger building, which can have an address from the adjacent street. Fixing zoom at 16 gives reliable street-level results.

On-demand address requests (for example, from the remote MCP `get_address` tool) use a two-tier approach. We first ask [Photon](https://photon.komoot.io/) for a building-level address with a house number, and if Photon errors or finds nothing we fall back to Nominatim for a street-only address. Automated tree address updates always stay street level via Nominatim.

A tree's address is normally updated when (1) a tree is added, and (2) a tree is updated which had no address set previously. If a tree already has an address set, it won't be auto-updated. If you see a tree with a wrong address, you can manually edit that tree and the address won't be overwritten by any automation.

You can use `curl` to test manual geocoding.

Street-level address with Nominatim:

```bash
curl -s "https://nominatim.openstreetmap.org/reverse?lat=40.1797541&lon=44.5106014&zoom=16&format=jsonv2&addressdetails=1&accept-language=en" | jq -s
[
  {
    "place_id": 193003973,
    "licence": "Data © OpenStreetMap contributors, ODbL 1.0. http://osm.org/copyright",
    "osm_type": "way",
    "osm_id": 304455525,
    "lat": "40.1797178",
    "lon": "44.5106317",
    "category": "highway",
    "type": "tertiary",
    "place_rank": 26,
    "importance": 0.05340703669794763,
    "addresstype": "road",
    "name": "Anton Kochinyan street",
    "display_name": "Anton Kochinyan street, Kentron, Yerevan, 0010, Armenia",
    "address": {
      "road": "Anton Kochinyan street",
      "suburb": "Kentron",
      "city": "Yerevan",
      "ISO3166-2-lvl4": "AM-ER",
      "postcode": "0010",
      "country": "Armenia",
      "country_code": "am"
    },
    "boundingbox": [
      "40.1797019",
      "40.1802878",
      "44.5106126",
      "44.5113105"
    ]
  }
]
```

Building-level address with Photon (English names require `lang=en`):

```bash
curl -s "https://photon.komoot.io/reverse?lat=40.204388&lon=44.524517&limit=1&lang=en" | jq
{
  "type": "FeatureCollection",
  "features": [
    {
      "type": "Feature",
      "properties": {
        "osm_type": "W",
        "osm_id": 82684027,
        "osm_key": "building",
        "osm_value": "retail",
        "type": "house",
        "housenumber": "13/1",
        "street": "Azatutyan Avenue",
        "district": "Arabkir",
        "city": "Yerevan",
        "country": "Armenia",
        "postcode": "0014",
        "countrycode": "AM",
        "extent": [
          44.5238704,
          40.2042554,
          44.5241578,
          40.2040143
        ]
      },
      "geometry": {
        "type": "Point",
        "coordinates": [
          44.523995,
          40.2041132
        ]
      }
    }
  ]
}
```
