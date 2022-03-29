# A mini extractor 

This is a small tool to extract the javascript and sql from the Thingworx XML file. It's very often you have to review a Thingworx design, but it would be very painful to load the design into a new system.

This tool will simply extract the javascript and/or sql script from the `Services` and `Subscription` tabs of all `Things`, `ThingTemplates` and `ThingShape`. 

It can also export all remote property bindings info, aiming for developer to validate the binding tag and `start type`.

## How to use

1. Download the executable file from the release page: [release](https://github.com/xudesheng/twxparser/releases). You can put this executable file in any folder presented in your path.

2. Two ways to export Thingworx design.

    1. Export the content to Source Control Entities:

        <img src="docs/image-20220328230435037.png" alt="image-20220328230435037" style="zoom:33%;" />

        

    2. Export the XML file from your Thingworx instance
        <img src="docs/image-20220305152509207.png" alt="image-20220305152509207" style="zoom:50%;" />

3. Execute the following command

    1. When you exported to a folder:

    ```
    twxparser -s <point to the folder you exported to> -e ./export_rootfolder/
    ```

    `-s` or `--source_path` indicates the file or folder you want to extract the scripts from.

    `-e` or `--export_root` indicates in which folder you want to export the scripts to.

    2. When you exported as a whole XML file:

       ```
       twxparser -s <your AllEntities.xml> -e ./export_rootfolder/
       ```

       

## Exported Result:

The exported content includes 3 folders and 1 csv file:

<img src="docs/image-20220328230250512.png" alt="image-20220328230250512" style="zoom:50%;" />

## Known Issue:

On windows, this tool will report some file can't be processed, the error message is:

```
Error processing file: "C:\\Users\\dxu\\PROD_SourceControl_Mar 22,2022\\Things\\BSLRG1003.xml"
The filename, directory name, or volume label syntax is incorrect. (os error 123)
```



Enjoy! You can post issues or improvement requests [here](https://github.com/xudesheng/twxparser/issues).

