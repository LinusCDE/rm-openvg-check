# rm-openvg-check

Just some foolery testing out whether OpenVG works on the reMarkable.

Make it compile:

- Download https://www.khronos.org/registry/OpenVG/ri/openvg-1.1-ri.zip
- Extract into this directory
- Move ri_package_1.1/ri/src/null/\*.cpp up one directory

## Current state

According to the graphic (first link in file `links`) the rM 1 should support OpenVG 1.1 and the rM 2 not.

Output of binary on rM 1 and rM 2:

```
reMarkable: /home/root/ ./rm-openvg-check
Extensions:
Opending display...
Opened display
Initialized display
Client APIs:
 - OpenVG
Vendor: Khronos Group
Version: 1.3
```

This is likely because this reference implementation will probably do the stuff on the CPU and not use the hardware. It should be possible to substitute the ri (reference implementation) with one from freescale/nxp for the IMX6/7. I didn't find one however. If someone finds it, please tell me / fork this. Otherwise this project will probably get stuck here.

If a lib can be found, I would like to build it for toltec as a shared lib, and create a rust lib for libremarkable (or integrate it directly) with the engoal being speeding up fitting rendering routines (just because).
