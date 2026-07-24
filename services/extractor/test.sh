mkdir -p odm_orthophoto odm_texturing

docker run -ti --rm \
  -v "$(pwd)/my_project:/project:z" \
  docker.io/opendronemap/odm \
  --skip-orthophoto --skip-report \
  --project-path /project .
