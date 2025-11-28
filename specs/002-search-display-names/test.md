# Ich möchte die browse Ansicht im frontend überarbeiten. 

## 1. Im Verzeichniss ./example ist ein Beispiel Projekt. Ich möchte es mehr wie ein Fileexplorer Funktioniert, indem ich Order für Ordner herabsteige und immer eine Vorschau der enthalten Projekte sehe. Wird beim Scannen eine Bilddatei gefunden soll sie immer auch in den darunter liegenden Projekten angezeigt / referenziert werden.

Als Beispiel enthält der Ordner "example/Miniaturen/The Printing Goes Ever On/Welcome Trove" das Bild "heroes fighting.jpg". Dies soll anzeigt werden wenn man auf der Ebene "example/Miniaturen/The Printing Goes Ever On" eine Vorschau von "Welcome Trove" sieht. Es soll aber auch eine jede Ebene darunter dem Projekt / Verzeichniss zugeordnet werden. Also "example/Miniaturen/The Printing Goes Ever On/Welcome Trove/Welcome-Trove-Remastered" und auch "example/Miniaturen/The Printing Goes Ever On/Welcome Trove/Welcome-Trove-Remastered". Doppelte

Doppelte Bildernamen müssen nicht weitervererbt werden / werden nur einmal angezeigt. 

Das Projekt "example/Miniaturen/The Printing Goes Ever On/Welcome Trove/Welcome-Trove-Remastered/Samuel" zeigt also auch "example/Miniaturen/The Printing Goes Ever On/Welcome Trove/heroes traveling and fighting.jpg" and, aber auch die eigenen Bilder wie z. B. "example/Miniaturen/The Printing Goes Ever On/Welcome Trove/Welcome-Trove-Remastered/Samuel/samuel fighting.jpg"


## 2. Die Keyword die Bestimmen ob ein Ordner ein Projekt ist, oder nur STL Dateien enthält die zum darüberliegenden Ordner gehören, sollen auch auf Containment geprüft werden.

Zum Beispiel "IGNORED_KEYWORDS=PRESUPPORTED_STL,STL,UNSUPPORTED_STL,Unsupported,Pre-Supported,inch,mm" enthält auch das Keyword inch. Ich möchte, dass damit auch der Ordner "example/Miniaturen/The Printing Goes Ever On/Welcome Trove/Universal-Base-Set/Desert/1 inch" als reines STL Verzeichnis und eine STl Kategorie in "example/Miniaturen/The Printing Goes Ever On/Welcome Trove/Universal-Base-Set/Desert" anezeigt wird.


## Das heißt das "example/Miniaturen/The Printing Goes Ever On/Welcome Trove/Universal-Base-Set/Desert" zukünftig ein Projekt ist, mit der STL Kategorie "1 inch" und "2 inch" und "40 mm" in dem auch Bilder wie "example/Miniaturen/The Printing Goes Ever On/Welcome Trove/heroes fighting.jpg" angezeigt werden.