if [ -z "$1" ]
  then
    echo "🚨 server args not supplied (dev, stage, prod) exiting!"
    exit 1
fi
if [ $1 = "dev" ] || [ $1 = "stage" ] || [ $1 = "prod" ]; then
    set -e
    REG_URL="apps.eposnet.uk"
    NAME="baqi"
    if [ $1 = "dev" ] || [ $1 = "stage" ]; then
        REG_URL="apps.eposnet.uk"
        NAME="$NAME"
    else
        echo "==> 🔒 loading PROD aws KEY"
    fi
    echo "==> 🔥 lets deploy the $NAME -> $REG_URL"
    docker tag $NAME "$REG_URL/$NAME"
    docker push "$REG_URL/$NAME"
else
    echo "🚨 server args does not match (dev, stage, prod) exiting!"
    exit 1
fi
