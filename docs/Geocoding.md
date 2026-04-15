# Geocoding

To automatically find street address for a tree, we use [Nominatim](https://nominatim.org/). We use zoom=16 to avoid address confusion introduced by building addresses, and only rely on closest streets.

A tree's address is normally updated when (1) a tree is added, and (2) a tree is updated which had no address set previously. If a tree already has an address set, it won't be auto-updated. If you see a tree with a wrong address, you can manually edit that tree and the address won't be overwritten by any automation.

You can use `curl` to test manual geocoding:

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
