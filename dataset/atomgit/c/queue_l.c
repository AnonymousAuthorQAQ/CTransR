#include <stdio.h>
#include <stdlib.h>

typedef int ElementType;

#define Error( Str )        FatalError( Str )
#define FatalError( Str )   fprintf( stderr, "%s\n", Str ), exit( 1 )

struct QueueRecord;
typedef struct QueueRecord *Queue;

int IsEmpty( Queue Q );
int IsFull( Queue Q );
Queue CreateQueue( int MaxElements );
void DisposeQueue( Queue Q );
void MakeEmpty( Queue Q );
void Enqueue( ElementType X, Queue Q );
ElementType Front( Queue Q );
void Dequeue( Queue Q );
ElementType FrontAndDequeue( Queue Q );

#define MinQueueSize ( 5 )

struct QueueRecord
{
    int Capacity;
    int Front;
    int Rear;
    int Size;
    ElementType *Array;
};

int IsEmpty( Queue Q )
{
    return Q->Size == 0;
}

int IsFull( Queue Q )
{
    return Q->Size == Q->Capacity;
}

Queue CreateQueue( int MaxElements )
{
    Queue Q;

    if( MaxElements < MinQueueSize )
        Error( "Queue size is too small" );

    Q = malloc( sizeof( struct QueueRecord ) );
    if( Q == NULL )
        FatalError( "Out of space!!!" );

    Q->Array = malloc( sizeof( ElementType ) * MaxElements );

    if( Q->Array == NULL )
        FatalError( "Out of space!!!" );

    Q->Capacity = MaxElements;
    MakeEmpty( Q );

    return Q;
}

void MakeEmpty( Queue Q )
{
    Q->Size = 0;
    Q->Front = 1;
    Q->Rear = 0;
}

void DisposeQueue( Queue Q )
{
    if( Q != NULL )
    {
        free( Q->Array );
        free( Q );
    }
}

static int Succ( int Value, Queue Q )
{
    if( ++Value == Q->Capacity )
        Value = 0;
    return Value;
}

void Enqueue( ElementType X, Queue Q )
{
    if( IsFull( Q ) )
        Error( "Full queue" );
    else
    {
        Q->Size++;
        Q->Rear = Succ( Q->Rear, Q );
        Q->Array[ Q->Rear ] = X;
    }
}

ElementType Front( Queue Q )
{
    if( !IsEmpty( Q ) )
        return Q->Array[ Q->Front ];
    Error( "Empty queue" );
    return 0; 
}

void Dequeue( Queue Q )
{
    if( IsEmpty( Q ) )
        Error( "Empty queue" );
    else
    {
        Q->Size--;
        Q->Front = Succ( Q->Front, Q );
    }
}

ElementType FrontAndDequeue( Queue Q )
{
    ElementType X = 0;

    if( IsEmpty( Q ) )
        Error( "Empty queue" );
    else
    {
        Q->Size--;
        X = Q->Array[ Q->Front ];
        Q->Front = Succ( Q->Front, Q );
    }
    return X;
}

int main( )
{
    Queue Q;
    int i, queueLength;
    int enqueueValue;

    if(scanf("%d", &queueLength) != 1 || queueLength < MinQueueSize)
    {
        fprintf(stderr, "Invalid input or queue size is too small\n");
        exit(1);
    }

    Q = CreateQueue(queueLength);

    // Test: Attempt to dequeue from an empty queue
    if (!IsEmpty(Q)) {
        Dequeue(Q);
    } else {
        printf("Queue is empty, cannot dequeue.\n");
    }

    // Enqueue elements and check if the queue is full
    for (i = 0; i < queueLength - 1; i++) {
        if (IsFull(Q)) {
            printf("Queue is full\n");
        } else {
            Enqueue(i, Q);
        }
    }

    // Test: Attempt to enqueue to a full queue
    if (!IsFull(Q)) {
        if (scanf("%d", &enqueueValue) == 1) {
            Enqueue(enqueueValue, Q);
        } else {
            fprintf(stderr, "Invalid input\n");
            exit(1);
        }
    } else {
        printf("Queue is full, cannot enqueue.\n");
    }

    // Use FrontAndDequeue to get and remove elements
    while (!IsEmpty(Q)) {
        printf("FrontAndDequeue: %d\n", FrontAndDequeue(Q));
    }

    // Enqueue elements again
    for (i = 0; i < queueLength - 2; i++) {
        Enqueue(i, Q);
    }

    // Use Front and Dequeue
    while (!IsEmpty(Q)) {
        printf("Front: %d\n", Front(Q));
        Dequeue(Q);
    }

    DisposeQueue(Q);
    return 0;
}
