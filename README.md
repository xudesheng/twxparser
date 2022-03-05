# A mini extractor 

This is a small tool to extract the javascript and sql from the Thingworx XML file. It's very often you have to review a Thingworx design, but it would be very painful to load the design into a new system.
This tool will simply extract the javascript and/or sql script from the `Services` and `Subscription` tabs of all `Things`, `ThingTemplates` and `ThingShape`. 

## How to use

1. Download the executable file from the release page: [release](https://github.com/xudesheng/twxparser/releases). You can put this executable file in any folder presented in your path.

1. Export the XML file from your Thingworx instance
 ![image-20220305151009751](C:\Users\dxu\AppData\Roaming\Typora\typora-user-images\image-20220305151009751.png)
 
3. Execute the following command

    ```
    twxparser -f ./sample/AllEntities.xml -r ./sample/
    ```

    `-f` or `--file-name` indicates the file you want to extract the scripts from.

    `-r` or `--root-path` indicates in which folder you want to export the scripts to.



Enjoy! You can post issues or improvement requests [here](https://github.com/xudesheng/twxparser/issues).

