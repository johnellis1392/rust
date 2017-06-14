# Collection of notes for relevant shell commands

####################################################
# Gets list of current ip addresses for the device #
#
# grep -E => Extended regex notation; same as egrep
# grep -o => Match exactly the pattern given
# grep -v => Invert matches; return all NOT matching pattern
ifconfig | grep -Eo 'inet (addr:)?([0-9]*\.){3}[0-9]*' | grep -Eo '([0-9]*\.){3}[0-9]*' | grep -v '127.0.0.1'

# Sed version
#
# sed -E => Extended regex notation
# sed -n => Disable automatic printing to stdout (all lines get printed automatically if not specified)
# s/../../ => Replace
# /../p => Print result
ifconfig | sed -En 's/127.0.0.1//;s/.*inet (addr:)?(([0-9]*\.){3}[0-9]*).*/\2/p'
