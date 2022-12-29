> Maybe entirely rework this part because the lib system is different

# STDLIB théorique du langage 
## Types

  - String :
    - Char (caractère unique) => Possibilité d'utiliser la table ASCII pour le générer
    - String (chaine de Char) => Possibilité de le générer via un Array de Char
  - Int :
    - Int64
    - Int32
    - 16? 8?
    - Unsigned64
    - Unisgned32
    - 16? 8?
  - Float :
    - Float64
    - Float32
    - 16? 8?
  - Array :
    - Static Array (On peut pas le modif) [Array Rust]|[Tuple Python]
    - Dynamic Array (On peut le modif) [List Python]|[Vec Rust]
    - Ensemble Array (Il ne peut contenir qu'une seule fois la meme valeur) [Set C]

  > Pas de dictionnaires mais des structs (implémentation à venir)

## stdlib Type method :
> Actuellement je ne sais pas si les fonctions agiront sur self ou juste sur les arguments, donc j'ai mis ? pour dire que ça peut être l'un ou l'autre

  - String :
    - x?.replaceString(x?, a, b)
      - Remplace tous les caractères a par les caractères b dans le string x
    - x?.splitStr(x?)
      - Transforme x dans un Static Array de char
    - x?.pushStr(x?, a)
      - Ajoute a à la fin de x
    - x?.len(x?)
      - Retourne la longueur du String x

  - Array :
    - x?.push(x?, a)
      - Ajoute a à la fin de x
        > Return ERROR si c'est un Static Array | Return SUCCESS si c'est un Dynamic Array | Return EXISTS ou SUCCESS si c'est un Ensemble array en fonction de si l'élément existe déjà ou non dedans
    - x?.len(x?)
      - Retourne la longueur de l'Array x
    - x?.remove(x?, idx)
      - Supprime l'élément à l'index idx de l'Array x
    - x?.pop(x?)
      - Retire le dernier élément de l'Array x
    - x?.sort(x?)
      - Arrange l'Array x (à détailler pour la manière)
    - x?.shuffle(x?)
      - Mélange l'Array x

  - Int | Float :
    - x?.div(x?, a)
      - Divise x par a
    - x?.mul(x?, a)
      - Multiplie x par a
    - x?.add(x?, a)
      - Additionne x avec a
    - x?.sub(x?, a)
      - Soustrait x par a
        > Si a = 0 Return ERROR et plante
    - x?.max(x?, a)
      - Return le plus grand des deux
        > Si ils sont équivalents, retourne x
    - x?.min(x?, a)
      - Return le plus petit des deux
        > Si ils sont équivalents, retourne x

  - Struct :
    > À venir...
  
  - Connaitre le type :
    - x?.typeOf(x?)
      - Retourne le type de x
    - x?.isType(x?, TYPE)
      - Retourne True si x est du type TYPE ou False si ce n'est pas le cas
## stdlib Basic Functions :
> Actuellement je ne sais pas si les fonctions agiront sur self ou juste sur les arguments, donc j'ai mis ? pour dire que ça peut être l'un ou l'autre

  - In Out :
    - print(x) 
      - Écrit x dans la console
    - println(x) 
      - Même chose que print mais avec une nouvelle ligne
    - read()
      - Lis l'input donné dans la console => Return un String

  - Casting :
    - x?.toString(x?)
      - Return x en String 
        > x peut être de tous les types (pour les structs/classes il faut implémenter soi même la méthode)
    - x?.toInt(x?)
      - Transforme x en Int
        > x ne peut pas être un Array ou une struct/classe (à moins d'implémenter la méthode)
        > Si x est un Char, ce sera sa valeur dans la table ascii qui sera retourné
    - x?.toChar(x?)
      - Transforme x en Char
        > x ne peut pas être un Array, un float ou une struct/classe (à moins d'implémenter la méthode)
        > Si x est un Int, ce sera sa valeur dans la table ascii qui sera retourné (Si il est trop grand, ERROR)
        > Si x est un String, seulement le premier Char sera pris en compte
    - x?.toFloat(x?)
      - Transforme x en Float
        > x ne peut qu'être un Char (Int => Float) ou un Int

  - File Access :
    > À venir...

  - Date :
    > À voir sur comment le structurer.. Soit un set de fonction tout seul ou lié à un objet

  - Basic functions :
    - random(a, b) 
      - Retourne un nombre aléatoire compris entre a et b
    - wait(x)
      - Mets en pause le programme pendant x temps
 