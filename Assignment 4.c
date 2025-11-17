#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <stdbool.h>
#include <string.h>
#define MATRIX_SIZE 5


// Define a (constant) 10x10 matrix data type

typedef struct {
    char indices[MATRIX_SIZE];
    int columns[MATRIX_SIZE][MATRIX_SIZE];
} DistanceMatrix;

typedef struct {
    char i, j;
    int pos_i, pos_j;
    float height;
    DistanceMatrix next_matrix;
} Cluster;


DistanceMatrix load_distance_matrix(char* filename) {
    DistanceMatrix matrix;

    FILE *fptr = fopen(filename, "r");
    if (!fptr) {
        perror("fopen");
        exit(1);
    }

    char line[100];
    char *token;
    int row = 0;
    int col = 0;
    bool header_line = true;

    while (fgets(line, sizeof(line), fptr)) {
        // Initial header parser
        if (header_line) {
            token = strtok(line, " ");
            int i = 0;
            while (token != NULL && i < MATRIX_SIZE) {
                matrix.indices[i] = token[0];
                printf("%c ", matrix.indices[i]);
                token = strtok(NULL, " ");
                i++;
            }
            header_line = false;
            printf("\n");
            continue;
        }
        // Numeric parser
        col = 0;
        token = strtok(line, " ");
        while (token != NULL) {
            matrix.columns[row][col] = atoi(token);
            //printf("\nRow %d, col %d: ", row, col);
            printf("%d ", matrix.columns[row][col]);
            token = strtok(NULL, " ");
            col++;
        }
        row++;
        printf("\n");
    }

    fclose(fptr);
    return matrix;
}

void upgma_algo(DistanceMatrix input) {
    Cluster clusters[50];
    int cluster_size = 0;
    // Find minimum
    int min_pos_i = 1;
    int min_pos_j = 0;
    int min = input.columns[min_pos_i][min_pos_j]; // Initialize non-empty minimum
    while (true) {
        for (int i = 0; i < MATRIX_SIZE; i++) {
            // Run across the diagonal
            for (int j = i + 1; j < MATRIX_SIZE; j++) {
                if (input.columns[i][j] < min) {
                    min_pos_i = i;
                    min_pos_j = j;
                    min = input.columns[i][j];
                    printf("Found at %d, %d", i, j);
                }
            }
        }
        // Create cluster at this position
        clusters[cluster_size].i = input.indices[min_pos_i];
        clusters[cluster_size].j = input.indices[min_pos_j];
        clusters[cluster_size].pos_i = min_pos_i;
        clusters[cluster_size].pos_j = min_pos_j;
        clusters[cluster_size].height = (float)min / 2;
        printf("Cluster created at %d, %d with characters %c, %c and height %f", min_pos_i, min_pos_j, clusters[cluster_size].i, clusters[cluster_size].j, clusters[cluster_size].height);
        cluster_size++;

        return;

    }


}


int main() {

    DistanceMatrix matrix = load_distance_matrix("A4_Bsp.txt");
    printf("\n\n");
    upgma_algo(matrix);

    return 0;
}