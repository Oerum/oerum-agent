# Quickstart (30 seconds)

## Windows

```powershell
irm https://raw.githubusercontent.com/Oerum/oerum-agent/main/install/bootstrap.ps1 | iex
brain init
```

## macOS/Linux

```sh
curl -fsSL https://raw.githubusercontent.com/Oerum/oerum-agent/main/install/bootstrap.sh | sh
brain init
```

Then run:

```sh
brain resume
```

If you get `404: Not Found` from the `raw.githubusercontent.com` URL, the
repository/branch/path may differ or the repo may be private. In that case,
run the script locally from a checked-out repo:

```powershell
.\install\bootstrap.ps1
brain init
```

```sh
./install/bootstrap.sh
brain init
```
