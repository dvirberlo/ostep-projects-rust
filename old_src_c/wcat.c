#include<stdio.h>
#include<stdlib.h>

int main(int argc, char *argv[]);
void catFile(char filename[]);

int main(int argc, char *argv[]){
    for (int i = 1; i < argc; i++){
        catFile(argv[i]);
    }
    return EXIT_SUCCESS;
}

void catFile(char filename[]){
    FILE *fp = fopen(filename, "r");
    if (fp == NULL) {
        printf("wcat: cannot open file\n");
        exit(1);
    }
    char *line  = NULL;
    size_t len  = 0;
    ssize_t read;
    while ((read = getline(&line, &len, fp)) != -1){
        printf("%s", line);
    }
    fclose(fp);
    if (line) free(line);
}
