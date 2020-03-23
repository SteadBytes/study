#!/usr/bin/env bash

set -e +x

TAP_NAME=tap-rust
TAP_CIDR=192.168.69.100/24
FORWARD_CIDR=192.168.69.0/24

function setup {
	# Create TAP device
	sudo ip tuntap add mode tap name $TAP_NAME user $USER

	# Assign IP to TAP device
	sudo ip link set "${TAP_NAME}" up
	# Note: This IP must match the IP expected by src/http.rs
	sudo ip addr add "${TAP_CIDR}" dev "${TAP_NAME}"
	cat << EOF
Created TAP device:
  Name: ${TAP_NAME}
  IP: ${TAP_CIDR}
EOF

	# Forward IPv4 packets
	sudo iptables -t nat -A POSTROUTING -s "${FORWARD_CIDR}" -j MASQUERADE
	sudo sysctl net.ipv4.ip_forward=1 &2> /dev/null
	echo Forwarding packets from "${FORWARD_CIDR}"
}

function teardown {
	sudo ip tuntap del mode tap name "${TAP_NAME}"
	cat << EOF
Removed TAP device:
  Name: ${TAP_NAME}
EOF
}

function usage {
	cat << EOF
Utility script to create/remove TAP device for running mget.

Sub-commands:
	setup - create TAP device '${TAP_NAME}' with IP ${TAP_CIDR} and forward packets from ${FORWARD_CIDR}
	teardown - Remove TAP device '${TAP_NAME}'
	-h|--help - prints this help
EOF
}

function main {
	if [ $# -eq 0 ]
		then
			usage
	fi
	local operation=$1
	shift
	case "$operation" in
			setup)setup $*;;
			teardown)teardown $*;;
			-h|--help)usage ;;
			*)usage ;;
    esac
}

main $@
